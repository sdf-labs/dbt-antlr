#![allow(clippy::mut_from_ref)]
use std::alloc::Layout;
use std::cell::{Cell, RefCell};
use std::ptr::{self, NonNull};

use crate::{
    cast_unchecked,
    token::{SourceInfo, Token},
    tree::{NodeInner, NodeKindType, TreeNode},
};

const DEFAULT_TOKENS_CAPACITY: usize = 1024;
const DEFAULT_CONTEXTS_CAPACITY: usize = 4096;
const TLS_MAX_RETAINED_BYTES: usize = 1024 * 1024;

thread_local! {
    static TLS_ARENA: Cell<Option<Arena>> = const { Cell::new(None) };
}

#[derive(Debug)]
pub struct Arena {
    tokens: bumpalo::Bump,
    contexts: bumpalo::Bump,
    primary_source_info: Cell<*mut SourceInfo<'static>>,
    source_infos: RefCell<Vec<*mut SourceInfo<'static>>>,
    #[cfg(feature = "arena-allocation-limit")]
    limit_bytes: usize,
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        self.drop_source_infos();
    }
}

macro_rules! check_allocation_limit {
    ($arena:expr) => {
        #[cfg(feature = "arena-allocation-limit")]
        {
            if $arena.limit_bytes > 0 && $arena.total_allocated_bytes() > $arena.limit_bytes {
                return std::ptr::null_mut();
            }
        }
    };
}

impl Arena {
    /// Invoke the callback with a reference to a scoped `Arena`.
    ///
    /// All allocations become invalid when the callback returns.
    /// The runtime clears and reuses the backing chunk on the current thread up to a retention cap.
    pub fn with<F, R>(f: F) -> R
    where
        F: for<'a> FnOnce(&'a Arena) -> R,
    {
        struct TlsArenaGuard(Option<Arena>);

        impl Drop for TlsArenaGuard {
            fn drop(&mut self) {
                if let Some(mut arena) = self.0.take() {
                    arena.reset_with_max_capacity(TLS_MAX_RETAINED_BYTES);
                    let _ = TLS_ARENA.try_with(|cell| {
                        cell.set(Some(arena));
                    });
                }
            }
        }

        let arena = TLS_ARENA
            .try_with(Cell::take)
            .ok()
            .flatten()
            .unwrap_or_default();

        #[cfg(feature = "arena-allocation-limit")]
        let arena = {
            let mut arena = arena;
            let limit = get_default_arena_alloc_limit();
            arena.limit_bytes = limit;
            if limit > 0 && arena.total_allocated_bytes() > limit {
                arena.tokens = bumpalo::Bump::new();
                arena.contexts = bumpalo::Bump::new();
            }
            arena
        };

        let guard = TlsArenaGuard(Some(arena));
        let arena_ref = guard.0.as_ref().expect("guard holds an active Arena");
        f(arena_ref)
    }

    #[cfg(feature = "arena-allocation-limit")]
    pub fn with_limit<F, R>(limit_bytes: usize, f: F) -> R
    where
        F: for<'a> FnOnce(&'a Arena) -> R,
    {
        let mut arena = Arena::with_capacity(0);
        arena.limit_bytes = limit_bytes;
        f(&arena)
    }

    /// Create a reusable `Arena` with the default initial chunk capacities.
    pub fn new() -> Self {
        #[cfg(feature = "arena-allocation-limit")]
        {
            let limit = get_default_arena_alloc_limit();
            let (tokens, contexts) = if limit > 0 {
                Self::bumps_within_cap(limit)
            } else {
                (
                    bumpalo::Bump::with_capacity(DEFAULT_TOKENS_CAPACITY),
                    bumpalo::Bump::with_capacity(DEFAULT_CONTEXTS_CAPACITY),
                )
            };
            Self {
                tokens,
                contexts,
                primary_source_info: Cell::new(ptr::null_mut()),
                source_infos: RefCell::new(Vec::new()),
                limit_bytes: limit,
            }
        }
        #[cfg(not(feature = "arena-allocation-limit"))]
        {
            Self {
                tokens: bumpalo::Bump::with_capacity(DEFAULT_TOKENS_CAPACITY),
                contexts: bumpalo::Bump::with_capacity(DEFAULT_CONTEXTS_CAPACITY),
                primary_source_info: Cell::new(ptr::null_mut()),
                source_infos: RefCell::new(Vec::new()),
            }
        }
    }

    /// Create a reusable `Arena` with an initial combined chunk capacity of at least `capacity_bytes`.
    pub fn with_capacity(capacity_bytes: usize) -> Self {
        let (tokens, contexts) = if capacity_bytes == 0 {
            (bumpalo::Bump::new(), bumpalo::Bump::new())
        } else {
            let tokens_cap = capacity_bytes / 4;
            let contexts_cap = capacity_bytes - tokens_cap;
            let tokens = if tokens_cap == 0 {
                bumpalo::Bump::new()
            } else {
                bumpalo::Bump::with_capacity(tokens_cap)
            };
            (tokens, bumpalo::Bump::with_capacity(contexts_cap))
        };
        Self {
            tokens,
            contexts,
            primary_source_info: Cell::new(ptr::null_mut()),
            source_infos: RefCell::new(Vec::new()),
            #[cfg(feature = "arena-allocation-limit")]
            limit_bytes: get_default_arena_alloc_limit(),
        }
    }

    fn bumps_within_cap(max_bytes: usize) -> (bumpalo::Bump, bumpalo::Bump) {
        if max_bytes >= DEFAULT_TOKENS_CAPACITY + DEFAULT_CONTEXTS_CAPACITY {
            let tokens = bumpalo::Bump::with_capacity(DEFAULT_TOKENS_CAPACITY);
            let contexts = bumpalo::Bump::with_capacity(DEFAULT_CONTEXTS_CAPACITY);
            if tokens.allocated_bytes() + contexts.allocated_bytes() <= max_bytes {
                return (tokens, contexts);
            }
        }
        (bumpalo::Bump::new(), bumpalo::Bump::new())
    }

    /// Clear all allocations in this arena and retain the largest bump chunks for reuse.
    pub fn reset(&mut self) {
        self.drop_source_infos();
        self.tokens.reset();
        self.contexts.reset();
        #[cfg(feature = "arena-allocation-limit")]
        if self.limit_bytes > 0 && self.total_allocated_bytes() > self.limit_bytes {
            (self.tokens, self.contexts) = Self::bumps_within_cap(self.limit_bytes);
        }
    }

    /// Clear all allocations and replace the retained chunks if their combined size exceeds `max_retained_bytes`.
    pub fn reset_with_max_capacity(&mut self, max_retained_bytes: usize) {
        self.drop_source_infos();
        self.tokens.reset();
        self.contexts.reset();
        #[cfg(feature = "arena-allocation-limit")]
        let effective_max = if self.limit_bytes > 0 {
            max_retained_bytes.min(self.limit_bytes)
        } else {
            max_retained_bytes
        };
        #[cfg(not(feature = "arena-allocation-limit"))]
        let effective_max = max_retained_bytes;

        if self.total_allocated_bytes() > effective_max {
            (self.tokens, self.contexts) = Self::bumps_within_cap(effective_max);
        }
    }

    /// Run `f` with a shared reference to this arena, then reset the arena on return or unwind.
    pub fn scope<F, R>(&mut self, f: F) -> R
    where
        F: for<'a> FnOnce(&'a Arena) -> R,
    {
        struct ResetOnDrop<'a>(&'a mut Arena);
        impl Drop for ResetOnDrop<'_> {
            fn drop(&mut self) {
                self.0.reset();
            }
        }
        let guard = ResetOnDrop(self);
        f(guard.0)
    }

    /// Run `f` with a shared reference to this arena, then reset with a retained-chunk cap.
    pub fn scope_with_max_capacity<F, R>(&mut self, max_retained_bytes: usize, f: F) -> R
    where
        F: for<'a> FnOnce(&'a Arena) -> R,
    {
        struct ResetCappedOnDrop<'a> {
            arena: &'a mut Arena,
            max_retained_bytes: usize,
        }
        impl Drop for ResetCappedOnDrop<'_> {
            fn drop(&mut self) {
                self.arena.reset_with_max_capacity(self.max_retained_bytes);
            }
        }
        let guard = ResetCappedOnDrop {
            arena: self,
            max_retained_bytes,
        };
        f(guard.arena)
    }

    fn drop_source_infos(&mut self) {
        let first = self.primary_source_info.get_mut();
        if !first.is_null() {
            let ptr = *first;
            *first = ptr::null_mut();
            // SAFETY: `ptr` lives in `self.tokens` and is dropped once before `self.tokens` resets or drops.
            unsafe {
                ptr::drop_in_place(ptr);
            }
        }
        for ptr in self.source_infos.get_mut().drain(..) {
            // SAFETY: `ptr` lives in `self.tokens` and is dropped once before `self.tokens` resets or drops.
            unsafe {
                ptr::drop_in_place(ptr);
            }
        }
    }

    #[inline]
    pub(crate) fn alloc_uninit_slice<T>(&self, len: usize) -> NonNull<T> {
        let layout = Layout::array::<T>(len).expect("slice layout overflow");
        self.contexts.alloc_layout(layout).cast::<T>()
    }

    #[inline(always)]
    pub fn alloc_token<T>(&self, value: T) -> &mut T
    where
        T: Token,
    {
        self.tokens.alloc(value)
    }

    pub fn alloc_string(&self, value: String) -> &str {
        self.tokens.alloc_str(value.as_str())
    }

    pub(crate) fn alloc_in_tokens<T>(&self, value: T) -> &T {
        self.tokens.alloc(value)
    }

    pub(crate) fn alloc_source_info<'a, 'input>(
        &'a self,
        value: SourceInfo<'input>,
    ) -> &'a SourceInfo<'input> {
        let ptr = self.tokens.alloc(value);
        let raw = (ptr as *mut SourceInfo<'input>).cast::<SourceInfo<'static>>();
        if self.primary_source_info.get().is_null() {
            self.primary_source_info.set(raw);
        } else {
            self.source_infos.borrow_mut().push(raw);
        }
        ptr
    }

    pub fn alloc_labeled_node<'input, 'a, N, I, Tok>(
        &self,
        value: I,
    ) -> *mut TreeNode<'input, 'a, N, Tok>
    where
        'input: 'a,
        N: NodeKindType<'a, Tok>,
        I: NodeInner<'input, 'a, N, Tok>,
        Tok: Token + 'input,
    {
        check_allocation_limit!(self);
        self.contexts.alloc(value) as *mut _ as *mut TreeNode<'input, 'a, N, Tok>
    }

    pub fn alloc_zeroed_node<'input, 'a, N, I, Tok>(
        &'a self,
        body: I,
    ) -> *mut TreeNode<'input, 'a, N, Tok>
    where
        'input: 'a,
        N: NodeKindType<'a, Tok>,
        I: NodeInner<'input, 'a, N, Tok>,
        Tok: Token + 'input,
    {
        check_allocation_limit!(self);
        let zeroed_header = unsafe { std::mem::zeroed::<TreeNode<'input, 'a, N, Tok>>() };
        let ptr = self.contexts.alloc((zeroed_header, body));
        ptr as *mut _ as *mut TreeNode<'input, 'a, N, Tok>
    }

    pub fn alloc_node<'input, 'a, N, I, Tok>(
        &'a self,
        header: TreeNode<'input, 'a, N, Tok>,
        body: I,
    ) -> *mut TreeNode<'input, 'a, N, Tok>
    where
        'input: 'a,
        N: NodeKindType<'a, Tok>,
        I: NodeInner<'input, 'a, N, Tok>,
        Tok: Token + 'input,
    {
        check_allocation_limit!(self);
        let ptr = self.contexts.alloc((header, body));
        // SAFETY: `header` is the first field of `(header, body)` and matches `TreeNode`.
        cast_unchecked!(ptr => mut TreeNode<'input, 'a, N, Tok>)
    }

    pub fn alloc_exception<'a, T>(&'a self, value: T) -> bumpalo::boxed::Box<'a, T> {
        bumpalo::boxed::Box::new_in(value, &self.contexts)
    }

    pub fn alloc_payload<T>(&self, value: T) -> &mut T {
        self.contexts.alloc(value)
    }

    pub fn total_allocated_bytes(&self) -> usize {
        self.tokens.allocated_bytes() + self.contexts.allocated_bytes()
    }

    #[cfg(feature = "arena-allocation-limit")]
    pub fn allocation_limit_bytes(&self) -> usize {
        self.limit_bytes
    }
}

pub(crate) fn is_ref_in_arena<T>(ptr: &T, arena: &bumpalo::Bump) -> bool {
    let p = ptr as *const T as usize;
    ptr_in_arena(p, arena)
}

pub(crate) fn is_slice_in_arena<T>(ptr: &[T], arena: &bumpalo::Bump) -> bool {
    let p = ptr.as_ptr() as usize;
    ptr_in_arena(p, arena)
}

fn ptr_in_arena(ptr: usize, arena: &bumpalo::Bump) -> bool {
    // SAFETY: No allocation runs on `arena` during chunk iteration.
    unsafe {
        arena.iter_allocated_chunks_raw().any(|(start, size)| {
            let start = start as usize;
            let end = start + size;
            ptr >= start && ptr < end
        })
    }
}

#[cfg(feature = "arena-allocation-limit")]
pub fn set_default_arena_alloc_limit(limit_bytes: usize) {
    ARENA_ALLOC_LIMIT.store(limit_bytes, std::sync::atomic::Ordering::SeqCst);
}

#[cfg(feature = "arena-allocation-limit")]
pub fn get_default_arena_alloc_limit() -> usize {
    ARENA_ALLOC_LIMIT.load(std::sync::atomic::Ordering::SeqCst)
}

#[cfg(feature = "arena-allocation-limit")]
static ARENA_ALLOC_LIMIT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cfg(test)]
mod tests {
    use std::panic::{catch_unwind, AssertUnwindSafe};

    use super::*;
    use crate::token::SourceInfo;

    #[test]
    fn arena_with_capacity_zero_starts_empty() {
        let arena = Arena::with_capacity(0);
        assert_eq!(arena.total_allocated_bytes(), 0);
    }

    #[test]
    fn arena_reset_reuses_chunk_and_drops_source_infos() {
        let mut arena = Arena::new();
        assert!(
            arena.total_allocated_bytes() >= DEFAULT_TOKENS_CAPACITY + DEFAULT_CONTEXTS_CAPACITY
        );

        for _ in 0..5 {
            arena.scope(|a| {
                let s1 = a.alloc_source_info(SourceInfo::new_stream("line1\nline2\n"));
                let s2 = a.alloc_source_info(SourceInfo::new_stream("other1\nother2\n"));
                let s3 = a.alloc_source_info(SourceInfo::new_stream("third1\nthird2\n"));
                assert_eq!(s1.line(6), 2);
                assert_eq!(s2.line(7), 2);
                assert_eq!(s3.line(7), 2);

                let buf = a.alloc_string("x".repeat(8192));
                assert_eq!(buf.len(), 8192);
                let _ = a.alloc_uninit_slice::<usize>(1024);
            });
        }

        let retained = arena.total_allocated_bytes();
        assert!(retained >= 8192 + 1024 * size_of::<usize>());
        arena.scope(|a| {
            let buf = a.alloc_string("y".repeat(4096));
            assert_eq!(buf.len(), 4096);
            let _ = a.alloc_uninit_slice::<usize>(512);
        });
        assert_eq!(arena.total_allocated_bytes(), retained);
    }

    #[test]
    fn arena_reset_with_max_capacity_caps_retained_chunk() {
        let mut arena = Arena::new();
        arena.scope_with_max_capacity(16 * 1024, |a| {
            let _ = a.alloc_string("a".repeat(64 * 1024));
            assert!(a.total_allocated_bytes() >= 64 * 1024);
        });
        assert!(arena.total_allocated_bytes() <= 16 * 1024);

        arena.reset_with_max_capacity(0);
        assert_eq!(arena.total_allocated_bytes(), 0);
    }

    #[test]
    fn arena_scope_resets_on_panic() {
        let mut arena = Arena::new();
        let res = catch_unwind(AssertUnwindSafe(|| {
            arena.scope(|a| {
                let s = a.alloc_source_info(SourceInfo::new_stream("a\nb\n"));
                assert_eq!(s.line(2), 2);
                panic!("simulated parse panic");
            });
        }));
        assert!(res.is_err());
        assert!(arena.primary_source_info.get().is_null());
        assert!(arena.source_infos.borrow().is_empty());

        arena.scope(|a| {
            assert_eq!(a.alloc_string("ok".to_owned()), "ok");
        });
    }

    #[test]
    fn arena_with_reuses_tls_handles_reentrancy_and_caps_growth() {
        Arena::with(|a| {
            let _ = a.alloc_string("w".repeat(32 * 1024));
        });

        let retained = Arena::with(|a| a.total_allocated_bytes());
        assert!(retained >= 32 * 1024);

        let inner_len = Arena::with(|outer| {
            let s1 = outer.alloc_string("outer".to_owned());
            let inner = Arena::with(|inner| inner.alloc_string("inner".to_owned()).len());
            assert_eq!(s1, "outer");
            inner
        });
        assert_eq!(inner_len, 5);

        let _ = catch_unwind(|| {
            Arena::with(|a| {
                let _ = a.alloc_source_info(SourceInfo::new_stream("x\ny\n"));
                panic!("tls panic");
            });
        });
        Arena::with(|a| {
            assert_eq!(a.alloc_string("recovered".to_owned()), "recovered");
        });

        Arena::with(|a| {
            let _ = a.alloc_string("b".repeat(TLS_MAX_RETAINED_BYTES + 64 * 1024));
        });
        let after_cap = Arena::with(|a| a.total_allocated_bytes());
        assert!(after_cap <= TLS_MAX_RETAINED_BYTES);
    }

    #[test]
    fn arena_reset_with_max_capacity_never_exceeds_small_cap() {
        let mut arena = Arena::new();
        arena.scope_with_max_capacity(5000, |a| {
            let _ = a.alloc_string("a".repeat(32 * 1024));
        });
        assert!(arena.total_allocated_bytes() <= 5000);
    }

    #[cfg(feature = "arena-allocation-limit")]
    #[test]
    fn arena_with_limit_is_not_poisoned_by_tls_chunk() {
        Arena::with(|a| {
            let _ = a.alloc_string("z".repeat(16 * 1024));
        });

        Arena::with_limit(512, |a| {
            assert_eq!(a.allocation_limit_bytes(), 512);
            assert_eq!(a.total_allocated_bytes(), 0);
        });
    }

    #[cfg(feature = "arena-allocation-limit")]
    #[test]
    #[serial_test::serial]
    fn arena_default_limit_shrinks_tls_and_reset_chunks() {
        struct ResetDefaultLimit;
        impl Drop for ResetDefaultLimit {
            fn drop(&mut self) {
                set_default_arena_alloc_limit(0);
            }
        }
        let _guard = ResetDefaultLimit;

        Arena::with(|a| {
            let _ = a.alloc_string("z".repeat(16 * 1024));
        });

        set_default_arena_alloc_limit(1024);

        Arena::with(|a| {
            assert_eq!(a.allocation_limit_bytes(), 1024);
            assert!(a.total_allocated_bytes() <= 1024);
            let source = a.alloc_source_info(SourceInfo::new_stream("a"));
            let tok = a.alloc_token(crate::token::CommonToken::new_slice(source, 1, 0, 0, 0, 0));
            let node = TreeNode::<crate::rule_context::EmptyNodeKind>::create_token_node(a, tok);
            assert!(!node.is_null());

            let _ = a.alloc_string("x".repeat(4096));
            let over_limit =
                TreeNode::<crate::rule_context::EmptyNodeKind>::create_token_node(a, tok);
            assert!(over_limit.is_null());
        });

        let mut reusable = Arena::new();
        assert_eq!(reusable.allocation_limit_bytes(), 1024);
        assert!(reusable.total_allocated_bytes() <= 1024);
        reusable.scope(|a| {
            let _ = a.alloc_string("y".repeat(4096));
            assert!(a.total_allocated_bytes() > 1024);
        });
        assert!(reusable.total_allocated_bytes() <= 1024);

        reusable.scope_with_max_capacity(64 * 1024, |a| {
            let source = a.alloc_source_info(SourceInfo::new_stream("b"));
            let tok = a.alloc_token(crate::token::CommonToken::new_slice(source, 1, 0, 0, 0, 0));
            let node = TreeNode::<crate::rule_context::EmptyNodeKind>::create_token_node(a, tok);
            assert!(!node.is_null());
            let _ = a.alloc_string("y".repeat(8192));
        });
        assert!(reusable.total_allocated_bytes() <= 1024);
    }
}
