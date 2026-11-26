#![allow(clippy::mut_from_ref)]
use std::alloc::Layout;
use std::cell::{Cell, RefCell};
use std::ptr::{self, NonNull};

use crate::{
    token::{SourceInfo, Token},
    tree::{NodeInner, NodeKindType, TreeNode},
};

const DEFAULT_TOKENS_CAPACITY: usize = 1024;
const DEFAULT_CONTEXTS_CAPACITY: usize = 4096;

#[repr(C)]
pub(crate) struct NodeAllocLayout<Header, Body> {
    pub(crate) header: Header,
    pub(crate) body: Body,
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

impl Drop for Arena {
    fn drop(&mut self) {
        let first = *self.primary_source_info.get_mut();
        if !first.is_null() {
            // SAFETY: `first` lives in `self.tokens` and is dropped once before `self.tokens` drops.
            unsafe {
                ptr::drop_in_place(first);
            }
        }
        for &mut ptr in self.source_infos.get_mut() {
            // SAFETY: `ptr` lives in `self.tokens` and is dropped once before `self.tokens` drops.
            unsafe {
                ptr::drop_in_place(ptr);
            }
        }
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
    /// Invoke the callback function with a reference to a local `Arena`.
    ///
    /// The arena drops when the callback returns.
    pub fn with<F, R>(f: F) -> R
    where
        F: for<'a> FnOnce(&'a Arena) -> R,
    {
        let arena = Arena::new();
        f(&arena)
    }

    #[cfg(feature = "arena-allocation-limit")]
    pub fn with_limit<F, R>(limit_bytes: usize, f: F) -> R
    where
        F: for<'a> FnOnce(&'a Arena) -> R,
    {
        let arena = Arena::new_with_limit(limit_bytes);
        f(&arena)
    }

    pub(crate) fn new() -> Self {
        #[cfg(feature = "arena-allocation-limit")]
        {
            Self::new_with_limit(get_default_arena_alloc_limit())
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

    #[cfg(feature = "arena-allocation-limit")]
    fn new_with_limit(limit_bytes: usize) -> Self {
        let (tokens, contexts) = if limit_bytes > 0 {
            (bumpalo::Bump::new(), bumpalo::Bump::new())
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
            limit_bytes,
        }
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

    #[cfg(feature = "ffi")]
    pub(crate) fn stream_source_info(&self, input: &str) -> *const std::ffi::c_void {
        let matches_input = |raw: *mut SourceInfo<'static>| -> bool {
            // SAFETY: Every non-null pointer in `primary_source_info` and
            // `source_infos` points to an initialized `SourceInfo` in `self.tokens`.
            let info = unsafe { &*raw };
            info.fallback.is_none()
                && info.input.as_ptr() == input.as_ptr()
                && info.input.len() == input.len()
        };
        let first = self.primary_source_info.get();
        if !first.is_null() && matches_input(first) {
            return first.cast();
        }
        for &raw in self.source_infos.borrow().iter() {
            if matches_input(raw) {
                return raw.cast();
            }
        }
        ptr::null()
    }

    #[inline]
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

    #[inline]
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
        let ptr = self.contexts.alloc(NodeAllocLayout {
            header: zeroed_header,
            body,
        });
        ptr as *mut _ as *mut TreeNode<'input, 'a, N, Tok>
    }

    #[inline]
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
        let ptr = self.contexts.alloc(NodeAllocLayout { header, body });
        ptr as *mut _ as *mut TreeNode<'input, 'a, N, Tok>
    }

    #[inline]
    pub(crate) fn alloc_node_children_store<'input, 'a, N, Tok>(
        &'a self,
        len: usize,
    ) -> NonNull<&'a TreeNode<'input, 'a, N, Tok>>
    where
        'input: 'a,
        N: NodeKindType<'a, Tok>,
        Tok: Token + 'input,
    {
        let layout =
            Layout::array::<&'a TreeNode<'input, 'a, N, Tok>>(len).expect("slice layout overflow");
        self.contexts
            .alloc_layout(layout)
            .cast::<&'a TreeNode<'input, 'a, N, Tok>>()
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

#[cfg(all(test, feature = "arena-allocation-limit"))]
mod tests {
    use super::*;
    use crate::rule_context::EmptyNodeKind;
    use crate::token::{CommonToken, SourceInfo};

    #[test]
    fn arena_with_limit_enforces_small_limit() {
        Arena::with_limit(512, |a| {
            assert_eq!(a.allocation_limit_bytes(), 512);
            assert_eq!(a.total_allocated_bytes(), 0);

            let source = a.alloc_source_info(SourceInfo::new_stream("a"));
            let tok = a.alloc_token(CommonToken::new_slice(source, 1, 0, 0, 0, 0));
            let node = TreeNode::<EmptyNodeKind>::create_token_node(a, tok);
            assert!(!node.is_null());

            let _ = a.alloc_string("x".repeat(4096));
            let over_limit = TreeNode::<EmptyNodeKind>::create_token_node(a, tok);
            assert!(over_limit.is_null());
        });
    }

    #[test]
    #[serial_test::serial]
    fn arena_default_limit_starts_within_limit() {
        struct ResetDefaultLimit;
        impl Drop for ResetDefaultLimit {
            fn drop(&mut self) {
                set_default_arena_alloc_limit(0);
            }
        }
        let _guard = ResetDefaultLimit;

        set_default_arena_alloc_limit(1024);

        Arena::with(|a| {
            assert_eq!(a.allocation_limit_bytes(), 1024);
            assert_eq!(a.total_allocated_bytes(), 0);
        });
    }
}
