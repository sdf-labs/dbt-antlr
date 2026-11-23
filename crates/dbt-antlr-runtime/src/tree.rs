//! General AST
use std::any::Any;

use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::{self, NonNull};
use std::slice;

use crate::char_stream::InputData;
use crate::errors::ANTLRError;
use crate::int_stream::EOF;
use crate::interval_set::Interval;
use crate::parser_rule_context::ParserRuleContext;
use crate::rule_context::RuleContext;
use crate::token::{CommonToken, Token};
use crate::{cast_unchecked, token_factory, Arena};

#[allow(missing_docs)]
pub trait Tree<'arena>: Sized {
    fn get_parent(&self) -> Option<&'arena Self>;

    fn has_parent(&self) -> bool;

    fn get_payload(&self) -> Box<dyn Any> {
        unimplemented!()
    }

    fn get_child(&self, _i: usize) -> Option<&'arena Self>;

    fn get_child_count(&self) -> usize;

    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item = &'arena Self> + 'a>;
}

/// Tree that knows about underlying text
pub trait ParseTree<'input, 'arena>: Tree<'arena> {
    /// Return an {@link Interval} indicating the index in the
    /// {@link TokenStream} of the first and last token associated with this
    /// subtree. If this node is a leaf, then the interval represents a single
    /// token and has interval i..i for token index i.
    fn get_source_interval(&self) -> Interval;

    /// Return combined text of this AST node.
    /// To create resulting string it does traverse whole subtree,
    /// also it includes only tokens added to the parse tree
    ///
    /// Since tokens on hidden channels (e.g. whitespace or comments) are not
    /// added to the parse trees, they will not appear in the output of this
    /// method.
    fn get_text(&self) -> String;
}

/// Helper trait, implemented for all rule context types that can be wrapped
/// inside [TreeNode].
pub trait NodeInner<'input, 'arena, Node, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    /// Extract a reference to self from a reference to [TreeNode].
    ///
    /// Used for safe downcasting.
    fn cast_from<'a>(node: &'a TreeNode<'input, 'arena, Node, Tok>) -> Option<&'a Self>
    where
        Self: Sized;

    /// Extract a reference to self from a reference to [TreeNode] without
    /// runtime tag checks.
    ///
    /// # Safety
    /// Caller must ensure `node` holds an instance of `Self`.
    #[inline(always)]
    unsafe fn cast_from_unchecked<'a>(node: &'a TreeNode<'input, 'arena, Node, Tok>) -> &'a Self
    where
        Self: Sized,
    {
        Self::cast_from(node).unwrap_unchecked()
    }

    /// Extract a mutable reference to self from a mutable reference to
    /// [TreeNode].
    ///
    /// Used for safe downcasting.
    fn cast_from_mut<'a>(node: &'a mut TreeNode<'input, 'arena, Node, Tok>) -> Option<&'a mut Self>
    where
        Self: Sized;

    /// Upcast &self back to a `&'arena Node`.
    fn as_node(&self) -> &TreeNode<'input, 'arena, Node, Tok>;

    fn as_node_mut(&mut self) -> &mut TreeNode<'input, 'arena, Node, Tok>;

    /// Iterate over children as [TreeNode]s
    fn iter_child_nodes<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'arena TreeNode<'input, 'arena, Node, Tok>> + 'a>;
}

pub type CtxPtr<'input, 'arena> = *mut (&'input (), *mut &'arena ());

pub trait NodeKindType<'arena, Tok = CommonToken<'arena>>:
    Copy + Debug + PartialEq + Eq + 'static
where
    Tok: Token + 'arena,
{
    type Listener: ParseTreeListener<'arena, Self, Tok> + ?Sized;

    fn cast_to_ctx<'n, 'input: 'arena>(
        node: &'n TreeNode<'input, 'arena, Self, Tok>,
    ) -> &'n dyn ParserRuleContext<'input, 'arena>;

    fn cast_to_ctx_mut<'n, 'input: 'arena>(
        node: &'n mut TreeNode<'input, 'arena, Self, Tok>,
    ) -> &'n mut dyn ParserRuleContext<'input, 'arena>;

    fn set_alt_number<'input: 'arena>(
        node: &mut TreeNode<'input, 'arena, Self, Tok>,
        alt_number: i32,
    );

    fn get_rule_index<'input: 'arena>(node: &TreeNode<'input, 'arena, Self, Tok>) -> usize;

    fn get_alt_number<'input: 'arena>(node: &TreeNode<'input, 'arena, Self, Tok>) -> i32;

    fn terminal() -> Self;

    fn error() -> Self;

    fn is_context(&self) -> bool;

    fn enter_rule<'input: 'arena>(
        node: &TreeNode<'input, 'arena, Self, Tok>,
        listener: &mut Self::Listener,
    ) -> Result<(), ANTLRError>;

    fn exit_rule<'input: 'arena>(
        node: &TreeNode<'input, 'arena, Self, Tok>,
        listener: &mut Self::Listener,
    ) -> Result<(), ANTLRError>;

    #[inline]
    fn is_terminal(&self) -> bool {
        *self == Self::terminal()
    }

    #[inline]
    fn is_error(&self) -> bool {
        *self == Self::error()
    }
}

const MAX_INLINE_CHILDREN: usize = 3;
const INITIAL_SPILL_CAPACITY: usize = 8;

#[repr(C)]
union ChildrenStore<'input, 'arena, NodeKind, Tok>
where
    'input: 'arena,
    NodeKind: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    inline: [MaybeUninit<&'arena TreeNode<'input, 'arena, NodeKind, Tok>>; MAX_INLINE_CHILDREN],
    spilled: NonNull<&'arena TreeNode<'input, 'arena, NodeKind, Tok>>,
}

#[repr(C)]
pub(crate) struct NodeChildren<'input, 'arena, NodeKind, Tok = CommonToken<'input>>
where
    'input: 'arena,
    NodeKind: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    len: u32,
    cap: u32,
    storage: ChildrenStore<'input, 'arena, NodeKind, Tok>,
}

impl<'input, 'arena, NodeKind, Tok> Debug for NodeChildren<'input, 'arena, NodeKind, Tok>
where
    'input: 'arena,
    NodeKind: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}

impl<'input, 'arena, NodeKind, Tok> NodeChildren<'input, 'arena, NodeKind, Tok>
where
    'input: 'arena,
    NodeKind: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    #[inline(always)]
    pub(crate) const fn new() -> Self {
        Self {
            len: 0,
            cap: 0,
            storage: ChildrenStore {
                inline: [MaybeUninit::uninit(); MAX_INLINE_CHILDREN],
            },
        }
    }

    #[inline(always)]
    pub(crate) fn len(&self) -> usize {
        self.len as usize
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline(always)]
    pub(crate) fn as_slice(&self) -> &[&'arena TreeNode<'input, 'arena, NodeKind, Tok>] {
        let ptr = if self.cap == 0 {
            // SAFETY: `cap == 0` selects the inline array, and `MaybeUninit<T>` has the same layout as `T`.
            unsafe {
                self.storage
                    .inline
                    .as_ptr()
                    .cast::<&'arena TreeNode<'input, 'arena, NodeKind, Tok>>()
            }
        } else {
            // SAFETY: `cap > 0` selects the spilled arena buffer.
            unsafe { self.storage.spilled.as_ptr().cast_const() }
        };
        // SAFETY: The first `self.len` elements in the active buffer are valid initialized references.
        unsafe { slice::from_raw_parts(ptr, self.len as usize) }
    }

    #[inline]
    pub(crate) fn get(&self, i: usize) -> Option<&'arena TreeNode<'input, 'arena, NodeKind, Tok>> {
        self.as_slice().get(i).copied()
    }

    #[inline]
    pub(crate) fn iter(&self) -> slice::Iter<'_, &'arena TreeNode<'input, 'arena, NodeKind, Tok>> {
        self.as_slice().iter()
    }

    #[inline]
    pub(crate) fn pop(&mut self) {
        if self.len > 0 {
            self.len -= 1;
        }
    }

    #[inline]
    pub(crate) fn truncate(&mut self, len: usize) {
        if len < self.len as usize {
            self.len = len as u32;
        }
    }

    #[inline]
    pub(crate) fn push(
        &mut self,
        child: &'arena TreeNode<'input, 'arena, NodeKind, Tok>,
        arena: &'arena Arena,
    ) {
        let len = self.len as usize;
        if self.cap == 0 {
            if len < MAX_INLINE_CHILDREN {
                // SAFETY: `cap == 0` selects the inline array, and `len` is less than `INLINE_CHILDREN`.
                unsafe {
                    self.storage.inline.get_unchecked_mut(len).write(child);
                }
                self.len = (len + 1) as u32;
                return;
            }
            self.spill_and_push(child, arena);
        } else if len < self.cap as usize {
            // SAFETY: `cap > 0` selects the spilled buffer with capacity `cap`,
            // and `len` is less than `cap`.
            unsafe {
                self.storage.spilled.as_ptr().add(len).write(child);
            }
            self.len = (len + 1) as u32;
        } else {
            self.grow_and_push(child, arena);
        }
    }

    #[cold]
    #[inline(never)]
    fn spill_and_push(
        &mut self,
        child: &'arena TreeNode<'input, 'arena, NodeKind, Tok>,
        arena: &'arena Arena,
    ) {
        debug_assert_eq!(self.cap, 0);
        debug_assert_eq!(self.len as usize, MAX_INLINE_CHILDREN);
        let dst = arena.alloc_node_children_store(INITIAL_SPILL_CAPACITY);
        // SAFETY: `cap == 0` selects the inline array with `INLINE_CHILDREN`
        // initialized elements, and `dst` holds `INITIAL_SPILL_CAPACITY`
        // elements.
        unsafe {
            ptr::copy_nonoverlapping(
                self.storage
                    .inline
                    .as_ptr()
                    .cast::<&'arena TreeNode<'input, 'arena, NodeKind, Tok>>(),
                dst.as_ptr(),
                MAX_INLINE_CHILDREN,
            );
            dst.as_ptr().add(MAX_INLINE_CHILDREN).write(child);
        }
        self.storage.spilled = dst;
        self.cap = INITIAL_SPILL_CAPACITY as u32;
        self.len = (MAX_INLINE_CHILDREN + 1) as u32;
    }

    #[cold]
    #[inline(never)]
    fn grow_and_push(
        &mut self,
        child: &'arena TreeNode<'input, 'arena, NodeKind, Tok>,
        arena: &'arena Arena,
    ) {
        debug_assert!(self.cap > 0);
        debug_assert_eq!(self.len, self.cap);
        let old_cap = self.cap as usize;
        let new_cap = self
            .cap
            .checked_mul(2)
            .expect("NodeChildren capacity overflow");
        let dst = arena.alloc_node_children_store(new_cap as usize);
        // SAFETY: `self.storage.spilled` holds `old_cap` initialized elements,
        // and `dst` holds `new_cap` elements.
        unsafe {
            ptr::copy_nonoverlapping(self.storage.spilled.as_ptr(), dst.as_ptr(), old_cap);
            dst.as_ptr().add(old_cap).write(child);
        }
        self.storage.spilled = dst;
        self.cap = new_cap;
        self.len = (old_cap + 1) as u32;
    }
}

#[repr(C, align(8))]
pub(crate) struct ParserRuleContextCommon<'input, 'arena, Node, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    pub(crate) start: Option<&'arena Tok>,
    pub(crate) stop: Option<&'arena Tok>,
    pub(crate) children: NodeChildren<'input, 'arena, Node, Tok>,
    pub(crate) parent: Option<&'arena TreeNode<'input, 'arena, Node, Tok>>,
}

impl<'input, 'arena, Node, Tok> NodeInner<'input, 'arena, Node, Tok>
    for ParserRuleContextCommon<'input, 'arena, Node, Tok>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    #[inline]
    fn cast_from<'a>(node: &'a TreeNode<'input, 'arena, Node, Tok>) -> Option<&'a Self> {
        node.as_rule_context_common()
    }

    #[inline(always)]
    unsafe fn cast_from_unchecked<'a>(node: &'a TreeNode<'input, 'arena, Node, Tok>) -> &'a Self {
        node.as_rule_context_common_unchecked()
    }

    #[inline]
    fn cast_from_mut<'a>(
        node: &'a mut TreeNode<'input, 'arena, Node, Tok>,
    ) -> Option<&'a mut Self> {
        node.as_rule_context_common_mut()
    }

    #[inline(always)]
    fn as_node(&self) -> &TreeNode<'input, 'arena, Node, Tok> {
        // SAFETY: All `ParserRuleContextCommon` instances sit at `TreeNode::body`
        // inside a `TreeNode` allocated by `Arena`.
        unsafe { TreeNode::from_ctx_ptr(self as *const Self as CtxPtr<'input, 'arena>) }
    }

    #[inline(always)]
    fn as_node_mut(&mut self) -> &mut TreeNode<'input, 'arena, Node, Tok> {
        // SAFETY: All `ParserRuleContextCommon` instances sit at `TreeNode::body`
        // inside a `TreeNode` allocated by `Arena`.
        unsafe { TreeNode::from_ctx_ptr_mut(self as *mut Self as CtxPtr<'input, 'arena>) }
    }

    fn iter_child_nodes<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'arena TreeNode<'input, 'arena, Node, Tok>> + 'a> {
        Box::new(self.children.iter().copied())
    }
}

const _: () = {
    use std::mem::{align_of, offset_of, size_of};

    use crate::parser_rule_context::BaseParserRuleContext;
    use crate::rule_context::{EmptyCustomRuleContext, EmptyNodeKind};

    type Common = ParserRuleContextCommon<'static, 'static, EmptyNodeKind>;
    type Base = BaseParserRuleContext<
        'static,
        'static,
        EmptyCustomRuleContext<'static, 'static>,
        EmptyNodeKind,
    >;
    type Node = TreeNode<'static, 'static, EmptyNodeKind>;

    assert!(size_of::<NodeChildren<'static, 'static, EmptyNodeKind>>() == 32);
    assert!(align_of::<NodeChildren<'static, 'static, EmptyNodeKind>>() == 8);
    assert!(size_of::<Common>() == 56);
    assert!(size_of::<Base>() == 56);
    assert!(size_of::<(Node, Base)>() == 64);
    assert!(size_of::<(Node, Common)>() == 64);
    assert!(align_of::<Common>() == align_of::<Base>());
    assert!(offset_of!((Node, Common), 1) == offset_of!((Node, Base), 1));
    assert!(offset_of!(Common, start) == offset_of!(Base, start));
    assert!(offset_of!(Common, stop) == offset_of!(Base, stop));
    assert!(offset_of!(Common, children) == offset_of!(Base, children));
    assert!(offset_of!(Common, parent) == offset_of!(Base, base));
};

#[cold]
#[inline(never)]
pub(crate) fn node_alloc_failure(arena: &Arena) -> ANTLRError {
    #[cfg(feature = "arena-allocation-limit")]
    {
        ANTLRError::arena_allocation_limit_exceeded(
            arena.allocation_limit_bytes(),
            arena.total_allocated_bytes(),
        )
    }
    #[cfg(not(feature = "arena-allocation-limit"))]
    {
        let _ = arena;
        std::alloc::handle_alloc_error(std::alloc::Layout::new::<
            TreeNode<'static, 'static, crate::rule_context::EmptyNodeKind>,
        >())
    }
}
#[derive(Debug)]
#[repr(C, align(8))]
pub struct TreeNode<'input, 'arena, NodeKind, Tok = CommonToken<'input>>
where
    'input: 'arena,
    NodeKind: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    pub(crate) label_tag: u16,
    pub node_tag: NodeKind,
    invoking_state: i32,
    body: (),

    _mark: PhantomData<(&'input Tok, *mut &'arena ())>,
}

impl<'input, 'arena, NodeKind, Tok> TreeNode<'input, 'arena, NodeKind, Tok>
where
    'input: 'arena,
    NodeKind: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    #[inline]
    pub fn create_token_node(arena: &'arena Arena, symbol: &'arena Tok) -> *mut Self {
        let header = TreeNode {
            node_tag: NodeKind::terminal(),
            label_tag: 0,
            invoking_state: -1,
            body: (),
            _mark: PhantomData,
        };
        let leaf = TerminalNode::new(symbol);
        arena.alloc_node(header, leaf)
    }

    #[allow(clippy::mut_from_ref)]
    #[inline]
    pub(crate) fn create_plain_context_node_with_start(
        arena: &'arena Arena,
        node_tag: NodeKind,
        parent: Option<&'arena Self>,
        invoking_state: i32,
        start: Option<&'arena Tok>,
    ) -> Result<&'arena mut Self, ANTLRError> {
        debug_assert!(node_tag.is_context());
        let header = TreeNode {
            node_tag,
            label_tag: 0,
            invoking_state,
            body: (),
            _mark: PhantomData,
        };
        let body = ParserRuleContextCommon {
            start,
            stop: None,
            children: NodeChildren::new(),
            parent,
        };
        let ptr = arena.alloc_node(header, body);
        if ptr.is_null() {
            return Err(node_alloc_failure(arena));
        }
        // SAFETY: `ptr` comes from `arena` for `'arena` with an initialized `TreeNode` header
        // and `ParserRuleContextCommon` body matching any plain ZST-extension `BaseParserRuleContext`.
        Ok(unsafe { &mut *ptr })
    }

    #[allow(clippy::mut_from_ref)]
    #[doc(hidden)]
    pub fn create_plain_context_node(
        arena: &'arena Arena,
        node_tag: NodeKind,
        parent: Option<&'arena Self>,
        invoking_state: i32,
    ) -> Result<&'arena mut Self, ANTLRError> {
        Self::create_plain_context_node_with_start(arena, node_tag, parent, invoking_state, None)
    }

    #[inline]
    pub fn create_error_node(arena: &'arena Arena, symbol: &'arena Tok) -> *mut Self {
        let header = TreeNode {
            node_tag: NodeKind::error(),
            label_tag: 0,
            invoking_state: -1,
            body: (),
            _mark: PhantomData,
        };
        let leaf = ErrorNode::new(symbol);
        arena.alloc_node(header, leaf)
    }

    #[inline]
    pub fn node_tag(&self) -> NodeKind {
        self.node_tag
    }

    /// Return the [ParserRuleContext] object associated with this rule node.
    pub fn get_rule_context(&self) -> &dyn ParserRuleContext<'input, 'arena> {
        NodeKind::cast_to_ctx(self)
    }

    /// Return mutable reference to the [ParserRuleContext] object associated
    /// with this rule node.
    pub fn get_rule_context_mut(&mut self) -> &mut dyn ParserRuleContext<'input, 'arena> {
        NodeKind::cast_to_ctx_mut(self)
    }

    #[inline]
    pub fn ctx_ptr(&self) -> CtxPtr<'input, 'arena> {
        &self.body as *const _ as *mut (&'input (), *mut &'arena ())
    }

    #[allow(dead_code)]
    pub(crate) fn get_stop_token(&self) -> Option<&'arena Tok> {
        self.as_rule_context_common().and_then(|ctx| ctx.stop)
    }

    /// # Safety
    /// The caller must make sure that `ctx_ptr` points to a valid [`NodeInner`]
    /// object in an arena:
    /// - Create leaf nodes with [`TreeNode::create_token_node`] or
    ///   [`TreeNode::create_error_node`].
    /// - Create rule context nodes with [`BaseParserRuleContext::create`],
    ///   [`TreeNode::create_plain_context_node`], or
    ///   [`TreeNode::create_plain_context_node_with_start`].
    #[inline]
    pub(crate) unsafe fn from_ctx_ptr(ctx_ptr: CtxPtr<'input, 'arena>) -> &'arena Self {
        &*((ctx_ptr as *const u8).sub(std::mem::offset_of!(Self, body)) as *const Self)
    }

    /// # Safety
    /// The caller must make sure that `ctx_ptr` points to a valid [`NodeInner`]
    /// object in an arena:
    /// - Create leaf nodes with [`TreeNode::create_token_node`] or
    ///   [`TreeNode::create_error_node`].
    /// - Create rule context nodes with [`BaseParserRuleContext::create`],
    ///   [`TreeNode::create_plain_context_node`], or
    ///   [`TreeNode::create_plain_context_node_with_start`].
    #[inline]
    pub(crate) unsafe fn from_ctx_ptr_mut(ctx_ptr: CtxPtr<'input, 'arena>) -> &'arena mut Self {
        &mut *((ctx_ptr as *const u8).sub(std::mem::offset_of!(Self, body)) as *mut Self)
    }

    #[inline]
    fn as_rule_context_common(
        &self,
    ) -> Option<&ParserRuleContextCommon<'input, 'arena, NodeKind, Tok>> {
        if self.node_tag.is_context() {
            Some(
                cast_unchecked!(self.ctx_ptr() => ParserRuleContextCommon<'input, 'arena, NodeKind, Tok>),
            )
        } else {
            None
        }
    }

    #[inline]
    fn as_rule_context_common_mut(
        &mut self,
    ) -> Option<&mut ParserRuleContextCommon<'input, 'arena, NodeKind, Tok>> {
        if self.node_tag.is_context() {
            Some(
                cast_unchecked!(self.ctx_ptr() => mut ParserRuleContextCommon<'input, 'arena, NodeKind, Tok>),
            )
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) unsafe fn as_rule_context_common_unchecked(
        &self,
    ) -> &ParserRuleContextCommon<'input, 'arena, NodeKind, Tok> {
        debug_assert!(self.node_tag.is_context());
        cast_unchecked!(self.ctx_ptr() => ParserRuleContextCommon<'input, 'arena, NodeKind, Tok>)
    }

    #[inline(always)]
    pub(crate) unsafe fn as_rule_context_common_mut_unchecked(
        &mut self,
    ) -> &mut ParserRuleContextCommon<'input, 'arena, NodeKind, Tok> {
        debug_assert!(self.node_tag.is_context());
        cast_unchecked!(self.ctx_ptr() => mut ParserRuleContextCommon<'input, 'arena, NodeKind, Tok>)
    }

    /// Downcast this node to a specific [ParserRuleContext] type without
    /// checking its node tag.
    ///
    /// # Safety
    /// Caller must ensure this node holds an instance of `T`.
    #[inline(always)]
    pub unsafe fn as_rule_context_unchecked<T>(&self) -> &T
    where
        T: ParserRuleContext<'input, 'arena> + NodeInner<'input, 'arena, NodeKind, Tok>,
    {
        T::cast_from_unchecked(self)
    }

    /// Attempt to downcast this node to specific [ParserRuleContext] type
    pub fn as_rule_context<T>(&self) -> Option<&T>
    where
        T: ParserRuleContext<'input, 'arena> + NodeInner<'input, 'arena, NodeKind, Tok>,
    {
        T::cast_from(self)
    }

    /// Attempt to downcast this node to specific [ParserRuleContext] type
    pub fn as_rule_context_mut<T>(&mut self) -> Option<&mut T>
    where
        T: ParserRuleContext<'input, 'arena> + NodeInner<'input, 'arena, NodeKind, Tok>,
    {
        T::cast_from_mut(self)
    }

    /// Downcast this node to terminal node, if the associated context is a
    /// terminal node
    pub fn as_terminal_node(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        TerminalNode::cast_from(self)
    }

    /// Downcast this node to terminal node, if the associated context is a
    /// terminal node
    pub fn as_terminal_node_mut(&mut self) -> Option<&mut TerminalNode<'input, 'arena, Tok>> {
        TerminalNode::cast_from_mut(self)
    }

    /// Downcast this node to error node, if the associated context is an error
    /// node
    pub fn as_error_node(&self) -> Option<&ErrorNode<'input, 'arena, Tok>> {
        ErrorNode::cast_from(self)
    }

    /// Downcast this node to error node, if the associated context is an error
    /// node
    pub fn as_error_node_mut(&mut self) -> Option<&mut ErrorNode<'input, 'arena, Tok>> {
        ErrorNode::cast_from_mut(self)
    }

    // pub fn add_token_node(&self, token: TerminalNode<'input, Self::TF>) { }
    // pub fn add_error_node(&self, bad_token: ErrorNode<'input, Self::TF>) { }

    pub fn set_exception(&self, _e: ANTLRError, _arena: &'arena Arena) {}

    /// Sets internal parser state
    pub fn set_invoking_state(&mut self, s: i32) {
        self.invoking_state = s;
    }

    pub fn set_alt_number(&mut self, _alt_number: i32) {
        NodeKind::set_alt_number(self, _alt_number);
    }

    pub fn set_start(&mut self, t: Option<&'arena Tok>) {
        let Some(ctx) = self.as_rule_context_common_mut() else {
            return;
        };

        ctx.start = t;
    }

    pub fn set_stop(&mut self, t: Option<&'arena Tok>) {
        let Some(ctx) = self.as_rule_context_common_mut() else {
            return;
        };

        ctx.stop = t;
    }

    pub fn remove_last_child(&mut self) {
        let Some(ctx) = self.as_rule_context_common_mut() else {
            return;
        };
        ctx.children.pop();
    }

    /// Drops every child after the first `len` children.
    pub fn truncate_children(&mut self, len: usize) {
        let Some(ctx) = self.as_rule_context_common_mut() else {
            return;
        };
        ctx.children.truncate(len);
    }

    /// Return the direct children of this node as a slice.
    ///
    /// Leaf nodes return an empty slice.
    #[inline]
    pub fn children_slice(&self) -> &[&'arena Self] {
        self.as_rule_context_common()
            .map_or(&[], |ctx| ctx.children.as_slice())
    }

    pub fn add_child(&mut self, child: &'arena Self, arena: &'arena Arena) {
        let Some(ctx) = self.as_rule_context_common_mut() else {
            return;
        };
        ctx.children.push(child, arena);
    }

    pub fn set_parent(&mut self, parent: Option<&'arena Self>) {
        let Some(ctx) = self.as_rule_context_common_mut() else {
            return;
        };
        ctx.parent = parent;
    }

    pub fn enter_rule(&self, listener: &mut NodeKind::Listener) -> Result<(), ANTLRError> {
        NodeKind::enter_rule(self, listener)
    }

    pub fn exit_rule(&self, listener: &mut NodeKind::Listener) -> Result<(), ANTLRError> {
        NodeKind::exit_rule(self, listener)
    }
}

impl<'input, 'arena, NodeKind, Tok> Tree<'arena> for TreeNode<'input, 'arena, NodeKind, Tok>
where
    'input: 'arena,
    NodeKind: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    #[inline]
    fn get_parent(&self) -> Option<&'arena Self> {
        self.as_rule_context_common().and_then(|ctx| ctx.parent)
    }

    fn has_parent(&self) -> bool {
        self.get_parent().is_some()
    }

    fn get_child(&self, i: usize) -> Option<&'arena Self> {
        self.as_rule_context_common()
            .and_then(|ctx| ctx.children.get(i))
    }

    #[inline]
    fn get_child_count(&self) -> usize {
        self.as_rule_context_common()
            .map_or(0, |ctx| ctx.children.len())
    }

    #[inline]
    fn get_children<'a>(&'a self) -> Box<dyn Iterator<Item = &'arena Self> + 'a> {
        if let Some(ctx) = self.as_rule_context_common() {
            Box::new(ctx.children.iter().copied())
        } else {
            Box::new(std::iter::empty())
        }
    }
}

impl<'input, 'arena, Node, Tok> ParseTree<'input, 'arena> for TreeNode<'input, 'arena, Node, Tok>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token,
{
    fn get_source_interval(&self) -> Interval {
        if let Some(terminal) = self.as_terminal_node() {
            Interval::new(
                terminal.symbol.get_start_index() as i32,
                terminal.symbol.get_stop_index() as i32,
            )
        } else if let Some(error) = self.as_error_node() {
            Interval::new(
                error.symbol.get_start_index() as i32,
                error.symbol.get_stop_index() as i32,
            )
        } else {
            let ctx = self
                .as_rule_context_common()
                .expect("Can only be rule context at this point");
            match (ctx.start, ctx.stop) {
                (Some(start), Some(stop)) => {
                    Interval::new(start.get_start_index() as i32, stop.get_stop_index() as i32)
                }
                _ => Interval::invalid(),
            }
        }
    }

    fn get_text(&self) -> String {
        if let Some(terminal) = self.as_terminal_node() {
            return terminal.get_text();
        } else if let Some(error) = self.as_error_node() {
            return error.get_text();
        }

        let mut result = String::new();
        for child in self.get_children() {
            result.push_str(&ParseTree::get_text(child));
        }
        result
    }
}

impl<'input, 'arena, NodeKind, Tok> RuleContext<'arena> for TreeNode<'input, 'arena, NodeKind, Tok>
where
    'input: 'arena,
    NodeKind: NodeKindType<'arena, Tok>,
    Tok: Token,
{
    fn get_rule_index(&self) -> usize {
        NodeKind::get_rule_index(self)
    }

    fn get_alt_number(&self) -> i32 {
        NodeKind::get_alt_number(self)
    }

    #[inline]
    fn get_invoking_state(&self) -> i32 {
        self.invoking_state
    }

    fn get_parent_ctx(&self) -> Option<&'arena dyn RuleContext<'arena>> {
        self.get_parent()
            .map(|p| p as &'arena dyn RuleContext<'arena>)
    }

    fn get_node_text(&self, rule_names: &[&str]) -> String {
        NodeKind::cast_to_ctx(self).get_node_text(rule_names)
    }
}

impl<'input, 'arena, Node, Tok> ParserRuleContext<'input, 'arena>
    for TreeNode<'input, 'arena, Node, Tok>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token,
{
    fn get_text(&self) -> String {
        ParseTree::get_text(self)
    }

    fn get_child_count(&self) -> usize {
        Tree::get_child_count(self)
    }

    fn start(&self) -> &'arena dyn Token {
        if let Some(terminal) = self.as_terminal_node() {
            return terminal.symbol;
        } else if let Some(error) = self.as_error_node() {
            return error.symbol;
        }
        let ctx = self
            .as_rule_context_common()
            .expect("Can only be rule context at this point");
        ctx.start.map_or_else(
            || token_factory::invalid() as &dyn Token,
            |t| t as &dyn Token,
        )
    }

    fn stop(&self) -> &'arena dyn Token {
        if let Some(terminal) = self.as_terminal_node() {
            return terminal.symbol;
        } else if let Some(error) = self.as_error_node() {
            return error.symbol;
        }
        let ctx = self
            .as_rule_context_common()
            .expect("Can only be rule context at this point");
        ctx.stop.map_or_else(
            || token_factory::invalid() as &dyn Token,
            |t| t as &dyn Token,
        )
    }

    fn get_parent_ctx(&self) -> Option<&'arena dyn ParserRuleContext<'input, 'arena>> {
        self.get_parent()
            .map(|p| p as &'arena dyn ParserRuleContext<'input, 'arena>)
    }

    fn get_child_ctx(&self, i: usize) -> Option<&'arena dyn ParserRuleContext<'input, 'arena>> {
        self.get_child(i)
            .map(|child| child.get_rule_context() as &dyn ParserRuleContext<'input, 'arena>)
    }

    fn iter_children<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'arena dyn ParserRuleContext<'input, 'arena>> + 'a>
    where
        'input: 'a,
        'arena: 'a,
    {
        let Some(ctx) = self.as_rule_context_common() else {
            return Box::new(std::iter::empty());
        };
        Box::new(
            ctx.children
                .iter()
                .map(|child| child.get_rule_context() as &dyn ParserRuleContext<'input, 'arena>),
        )
    }

    fn get_token(&self, _ttype: i32, _pos: usize) -> Option<&dyn Token> {
        let ctx = self.as_rule_context_common()?;
        ctx.children
            .iter()
            .filter_map(|child| child.as_terminal_node())
            .filter(|t| t.symbol.get_token_type() == _ttype)
            .nth(_pos)
            .map(|t| t.symbol as &dyn Token)
    }

    fn get_tokens(&self, _ttype: i32) -> Vec<&dyn Token> {
        let Some(ctx) = self.as_rule_context_common() else {
            return vec![];
        };
        ctx.children
            .iter()
            .filter_map(|child| child.as_terminal_node())
            .filter(|t| t.symbol.get_token_type() == _ttype)
            .map(|t| t.symbol as &dyn Token)
            .collect()
    }

    fn to_string_tree(&self, rule_names: &[&str]) -> String {
        crate::trees::string_tree(self, rule_names)
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub struct NoError;

#[doc(hidden)]
#[derive(Debug)]
pub struct IsError;

/// Generic leaf AST node
pub struct LeafNodeInner<'input, 'arena, E, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
    E: 'static,
{
    /// Token, this leaf consist of
    pub symbol: &'arena Tok,
    iserror: PhantomData<(E, &'input ())>,
}

impl<'input, 'arena, E, Tok> RuleContext<'arena> for LeafNodeInner<'input, 'arena, E, Tok>
where
    'input: 'arena,
    E: 'static,
    Tok: Token + 'input,
{
    fn get_rule_index(&self) -> usize {
        usize::MAX
    }

    fn get_alt_number(&self) -> i32 {
        crate::atn::INVALID_ALT
    }

    fn get_node_text(&self, _rule_names: &[&str]) -> String {
        self.symbol.get_text().to_display()
    }

    fn get_invoking_state(&self) -> i32 {
        -1
    }

    fn get_parent_ctx(&self) -> Option<&'arena dyn RuleContext<'arena>> {
        None
    }
}

impl<'input, 'arena, E, Tok> ParserRuleContext<'input, 'arena>
    for LeafNodeInner<'input, 'arena, E, Tok>
where
    'input: 'arena,
    E: 'static,
    Tok: Token + 'input,
{
    fn get_text(&self) -> String {
        self.symbol.get_text().to_display()
    }

    fn get_child_count(&self) -> usize {
        0
    }

    fn iter_children<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'arena dyn ParserRuleContext<'input, 'arena>> + 'a>
    where
        'input: 'a,
        'arena: 'a,
    {
        Box::new(std::iter::empty())
    }

    fn get_token(&self, _ttype: i32, _pos: usize) -> Option<&dyn Token> {
        None
    }

    fn get_tokens(&self, _ttype: i32) -> Vec<&dyn Token> {
        vec![]
    }
}

impl<'input, 'arena, Node, Tok> NodeInner<'input, 'arena, Node, Tok>
    for LeafNodeInner<'input, 'arena, NoError, Tok>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    fn cast_from<'a>(node: &'a TreeNode<'input, 'arena, Node, Tok>) -> Option<&'a Self>
    where
        Self: Sized,
    {
        if node.node_tag.is_terminal() {
            Some(unsafe { &*(node.ctx_ptr() as *const Self) })
        } else {
            None
        }
    }

    fn cast_from_mut<'a>(node: &'a mut TreeNode<'input, 'arena, Node, Tok>) -> Option<&'a mut Self>
    where
        Self: Sized,
    {
        if node.node_tag.is_terminal() {
            Some(unsafe { &mut *(node.ctx_ptr() as *mut Self) })
        } else {
            None
        }
    }

    fn as_node(&self) -> &TreeNode<'input, 'arena, Node, Tok> {
        // SAFETY: All [NodeInner] instances are allocated by
        // [Arena::alloc_node] with a valid [RuleNodeImpl] header
        unsafe { TreeNode::from_ctx_ptr(self as *const Self as CtxPtr<'input, 'arena>) }
    }

    fn as_node_mut(&mut self) -> &mut TreeNode<'input, 'arena, Node, Tok> {
        // SAFETY: All [NodeInner] instances are allocated by
        // [Arena::alloc_node] with a valid [RuleNodeImpl] header
        unsafe { TreeNode::from_ctx_ptr_mut(self as *mut Self as CtxPtr<'input, 'arena>) }
    }

    fn iter_child_nodes<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'arena TreeNode<'input, 'arena, Node, Tok>> + 'a> {
        Box::new(std::iter::empty())
    }
}

impl<'input, 'arena, Node, Tok> NodeInner<'input, 'arena, Node, Tok>
    for LeafNodeInner<'input, 'arena, IsError, Tok>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    fn cast_from<'a>(node: &'a TreeNode<'input, 'arena, Node, Tok>) -> Option<&'a Self>
    where
        Self: Sized,
    {
        if node.node_tag.is_error() {
            Some(unsafe { &*(node.ctx_ptr() as *const Self) })
        } else {
            None
        }
    }

    fn cast_from_mut<'a>(node: &'a mut TreeNode<'input, 'arena, Node, Tok>) -> Option<&'a mut Self>
    where
        Self: Sized,
    {
        if node.node_tag.is_error() {
            Some(unsafe { &mut *(node.ctx_ptr() as *mut Self) })
        } else {
            None
        }
    }

    fn as_node(&self) -> &TreeNode<'input, 'arena, Node, Tok> {
        // SAFETY: All [NodeInner] instances are allocated by
        // [Arena::alloc_node] with a valid [RuleNodeImpl] header
        unsafe { TreeNode::from_ctx_ptr(self as *const Self as CtxPtr<'input, 'arena>) }
    }

    fn as_node_mut(&mut self) -> &mut TreeNode<'input, 'arena, Node, Tok> {
        // SAFETY: All [NodeInner] instances are allocated by
        // [Arena::alloc_node] with a valid [RuleNodeImpl] header
        unsafe { TreeNode::from_ctx_ptr_mut(self as *mut Self as CtxPtr<'input, 'arena>) }
    }

    fn iter_child_nodes<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = &'arena TreeNode<'input, 'arena, Node, Tok>> + 'a> {
        Box::new(std::iter::empty())
    }
}

impl<'input, 'arena, E, Tok> Debug for LeafNodeInner<'input, 'arena, E, Tok>
where
    E: 'static,
    Tok: Token + 'input,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.symbol.get_token_type() == EOF {
            f.write_str("<EOF>")
        } else {
            let a = self.symbol.get_text().to_display();
            f.write_str(&a)
        }
    }
}

impl<'input, 'arena, E, Tok> LeafNodeInner<'input, 'arena, E, Tok>
where
    'input: 'arena,
    E: 'static,
    Tok: Token + 'input,
{
    /// Creates new leaf node
    ///
    /// Note: [LeafNodeInner] can not exist as a standalone entity, this
    /// constructor can only be called from [RuleNodeImpl::create_token_node] or
    /// [RuleNodeImpl::create_error_node]
    fn new(symbol: &'arena Tok) -> Self {
        Self {
            symbol,
            iserror: Default::default(),
        }
    }
}

/// non-error AST leaf node
pub type TerminalNode<'input, 'arena, Tok = CommonToken<'input>> =
    LeafNodeInner<'input, 'arena, NoError, Tok>;

/// # Error Leaf
/// Created for each token created or consumed during recovery
pub type ErrorNode<'input, 'arena, Tok = CommonToken<'input>> =
    LeafNodeInner<'input, 'arena, IsError, Tok>;

pub trait ParseTreeVisitor<'input, 'arena, Node, Tok>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    type Return: Default;

    fn visit(
        &mut self,
        node: &TreeNode<'input, 'arena, Node, Tok>,
    ) -> Result<Self::Return, ANTLRError>;

    /// Called on terminal(leaf) node
    fn visit_terminal(
        &mut self,
        _node: &TerminalNode<'input, 'arena, Tok>,
    ) -> Result<Self::Return, ANTLRError> {
        Ok(Self::Return::default())
    }

    /// Called on error node
    fn visit_error_node(
        &mut self,
        _node: &ErrorNode<'input, 'arena, Tok>,
    ) -> Result<Self::Return, ANTLRError> {
        Ok(Self::Return::default())
    }

    fn visit_children(
        &mut self,
        node: &dyn NodeInner<'input, 'arena, Node, Tok>,
    ) -> Result<Self::Return, ANTLRError> {
        let mut result = Self::Return::default();
        for child in node.iter_child_nodes() {
            if !self.should_visit_next_child(child, &result) {
                break;
            }

            let child_result = self.visit(child)?;
            result = self.aggregate_results(result, child_result)?;
        }
        Ok(result)
    }

    fn aggregate_results(
        &self,
        _aggregate: Self::Return,
        next: Self::Return,
    ) -> Result<Self::Return, ANTLRError> {
        Ok(next)
    }

    fn should_visit_next_child(
        &self,
        _node: &TreeNode<'input, 'arena, Node, Tok>,
        _current: &Self::Return,
    ) -> bool {
        true
    }
}

/// Base parse listener interface
pub trait ParseTreeListener<'arena, Node, Tok>
where
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'arena,
{
    /// Called when parser creates terminal node
    fn visit_terminal(&mut self, _node: &TerminalNode<'_, 'arena, Tok>) -> Result<(), ANTLRError> {
        Ok(())
    }

    /// Called when parser creates error node
    fn visit_error_node(&mut self, _node: &ErrorNode<'_, 'arena, Tok>) -> Result<(), ANTLRError> {
        Ok(())
    }

    /// Called when parser enters any rule node
    fn enter_every_rule(
        &mut self,
        _ctx: &TreeNode<'_, 'arena, Node, Tok>,
    ) -> Result<(), ANTLRError> {
        Ok(())
    }

    /// Called when parser exits any rule node
    fn exit_every_rule(
        &mut self,
        _ctx: &TreeNode<'_, 'arena, Node, Tok>,
    ) -> Result<(), ANTLRError> {
        Ok(())
    }
}

/// Helper struct to accept parse listener on already generated tree
#[derive(Debug)]
pub struct ParseTreeWalker;

impl ParseTreeWalker {
    /// Walks recursively over tree `t` with `listener`
    pub fn walk<'input, 'arena, Node, Tok>(
        mut listener: Box<Node::Listener>,
        t: &TreeNode<'input, 'arena, Node, Tok>,
    ) -> Result<Box<Node::Listener>, ANTLRError>
    where
        'input: 'arena,
        Node: NodeKindType<'arena, Tok>,
        Tok: Token + 'input,
    {
        if let Some(terminal) = t.as_terminal_node() {
            listener.visit_terminal(terminal)?;
            return Ok(listener);
        }
        if let Some(error) = t.as_error_node() {
            listener.visit_error_node(error)?;
            return Ok(listener);
        }

        Self::enter_rule(&mut *listener, t)?;
        for child in t.get_children() {
            listener = Self::walk(listener, child)?;
        }
        Self::exit_rule(&mut *listener, t)?;

        Ok(listener)
    }

    fn enter_rule<'input, 'arena, Node, Tok>(
        listener: &mut Node::Listener,
        t: &TreeNode<'input, 'arena, Node, Tok>,
    ) -> Result<(), ANTLRError>
    where
        'input: 'arena,
        Node: NodeKindType<'arena, Tok>,
        Tok: Token + 'input,
    {
        listener.enter_every_rule(t)?;
        t.enter_rule(listener)
    }

    fn exit_rule<'input, 'arena, Node, Tok>(
        listener: &mut Node::Listener,
        t: &TreeNode<'input, 'arena, Node, Tok>,
    ) -> Result<(), ANTLRError>
    where
        'input: 'arena,
        Node: NodeKindType<'arena, Tok>,
        Tok: Token + 'input,
    {
        t.exit_rule(listener)?;
        listener.exit_every_rule(t)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::{align_of, offset_of, size_of};

    use super::*;
    use crate::parser_rule_context::BaseParserRuleContext;
    use crate::rule_context::{EmptyCustomRuleContext, EmptyNodeKind};
    use crate::token::CommonToken;

    fn make_token<'arena>(
        arena: &'arena Arena,
        source: &'arena crate::token::SourceInfo<'arena>,
        idx: i32,
    ) -> &'arena CommonToken<'arena> {
        arena.alloc_token(CommonToken::new_slice(source, 1, 0, idx, 0, 0))
    }

    fn make_terminal<'arena>(
        arena: &'arena Arena,
        source: &'arena crate::token::SourceInfo<'arena>,
        idx: i32,
    ) -> &'arena TreeNode<'arena, 'arena, EmptyNodeKind, CommonToken<'arena>> {
        let tok = make_token(arena, source, idx);
        let ptr = TreeNode::<EmptyNodeKind, CommonToken<'arena>>::create_token_node(arena, tok);
        // SAFETY: `create_token_node` allocates a valid `TreeNode` in `arena`.
        unsafe { &*ptr }
    }

    #[test]
    fn layout_matches_base_parser_rule_context() {
        type Common<'i, 'a> = ParserRuleContextCommon<'i, 'a, EmptyNodeKind>;
        type Base<'i, 'a> =
            BaseParserRuleContext<'i, 'a, EmptyCustomRuleContext<'i, 'a>, EmptyNodeKind>;
        type Node<'i, 'a> = TreeNode<'i, 'a, EmptyNodeKind>;

        assert_eq!(size_of::<NodeChildren<'_, '_, EmptyNodeKind>>(), 32);
        assert_eq!(align_of::<NodeChildren<'_, '_, EmptyNodeKind>>(), 8);
        assert_eq!(size_of::<Common<'_, '_>>(), 56);
        assert_eq!(size_of::<Base<'_, '_>>(), 56);
        assert_eq!(size_of::<(Node<'_, '_>, Base<'_, '_>)>(), 64);

        assert_eq!(
            offset_of!(Common<'_, '_>, start),
            offset_of!(Base<'_, '_>, start)
        );
        assert_eq!(
            offset_of!(Common<'_, '_>, stop),
            offset_of!(Base<'_, '_>, stop)
        );
        assert_eq!(
            offset_of!(Common<'_, '_>, children),
            offset_of!(Base<'_, '_>, children)
        );
        assert_eq!(
            offset_of!(Common<'_, '_>, parent),
            offset_of!(Base<'_, '_>, base)
        );

        #[repr(C, u16)]
        #[allow(dead_code)]
        enum LabeledAll<'i, 'a> {
            Alt1(Base<'i, 'a>),
            Error(Base<'i, 'a>),
        }

        assert_eq!(size_of::<LabeledAll<'_, '_>>(), 64);
        assert_eq!(align_of::<LabeledAll<'_, '_>>(), 8);
        assert_eq!(offset_of!(Node<'_, '_>, body), 8);
    }

    #[test]
    fn node_children_inline_spill_grow_pop_and_truncate() {
        Arena::with(|arena| {
            let source = arena.alloc_source_info(crate::token::SourceInfo::new_stream("test"));
            let mut children = NodeChildren::<EmptyNodeKind>::new();

            assert!(children.is_empty());
            assert_eq!(children.len(), 0);
            assert_eq!(children.cap, 0);
            assert!(children.as_slice().is_empty());
            assert!(children.get(0).is_none());

            children.pop();
            children.truncate(5);
            assert_eq!(children.len(), 0);

            let nodes: Vec<_> = (0..35).map(|i| make_terminal(arena, source, i)).collect();

            for (idx, &node) in nodes.iter().take(MAX_INLINE_CHILDREN).enumerate() {
                children.push(node, arena);
                assert_eq!(children.cap, 0);
                assert_eq!(children.len(), idx + 1);
                assert!(ptr::eq(children.get(idx).unwrap(), node));
            }
            assert!(children.get(MAX_INLINE_CHILDREN).is_none());

            children.truncate(10);
            assert_eq!(children.len(), 3);
            children.pop();
            assert_eq!(children.len(), 2);
            children.truncate(1);
            assert_eq!(children.len(), 1);
            assert_eq!(children.cap, 0);
            assert!(ptr::eq(children.get(0).unwrap(), nodes[0]));

            children.push(nodes[1], arena);
            children.push(nodes[2], arena);
            assert_eq!(children.cap, 0);
            assert_eq!(children.len(), 3);

            children.push(nodes[3], arena);
            assert_eq!(children.cap as usize, INITIAL_SPILL_CAPACITY);
            assert_eq!(children.len(), 4);

            for &node in &nodes[4..INITIAL_SPILL_CAPACITY] {
                children.push(node, arena);
                assert_eq!(children.cap as usize, INITIAL_SPILL_CAPACITY);
            }
            assert_eq!(children.len(), INITIAL_SPILL_CAPACITY);

            children.push(nodes[8], arena);
            assert_eq!(children.cap, 16);
            for &node in &nodes[9..] {
                children.push(node, arena);
            }
            assert_eq!(children.len(), 35);
            assert_eq!(children.cap, 64);

            for (i, &expected) in nodes.iter().enumerate() {
                assert!(ptr::eq(children.get(i).unwrap(), expected));
            }
            assert!(children.get(35).is_none());
            assert_eq!(children.iter().count(), 35);

            children.truncate(100);
            assert_eq!(children.len(), 35);
            children.truncate(5);
            assert_eq!(children.len(), 5);
            assert_eq!(children.cap, 64);
            children.pop();
            assert_eq!(children.len(), 4);
            children.push(nodes[4], arena);
            assert_eq!(children.len(), 5);
            assert_eq!(children.cap, 64);
            for (i, &expected) in nodes.iter().take(5).enumerate() {
                assert!(ptr::eq(children.get(i).unwrap(), expected));
            }
        });
    }

    #[test]
    fn tree_node_and_context_children_api() {
        Arena::with(|arena| {
            let source = arena.alloc_source_info(crate::token::SourceInfo::new_stream("abc"));
            let root =
                BaseParserRuleContext::create(arena, None, -1, EmptyCustomRuleContext(PhantomData))
                    .expect("create root");

            let leaves: Vec<_> = (0..10).map(|i| make_terminal(arena, source, i)).collect();
            for &leaf in &leaves {
                root.add_child(leaf, arena);
            }

            assert_eq!(Tree::get_child_count(root), 10);
            assert_eq!(root.get_children().count(), 10);
            for (i, &leaf) in leaves.iter().enumerate() {
                assert!(ptr::eq(root.get_child(i).unwrap(), leaf));
            }

            let ctx = root
                .as_rule_context::<BaseParserRuleContext<EmptyCustomRuleContext, EmptyNodeKind>>()
                .unwrap();
            assert_eq!(ctx.get_children().count(), 10);
            assert_eq!(ctx.iter_children().count(), 10);
            assert_eq!(root.children_slice().len(), 10);
            assert_eq!(ctx.children_slice().len(), 10);
            assert!(leaves[0].children_slice().is_empty());

            root.remove_last_child();
            assert_eq!(Tree::get_child_count(root), 9);
            root.truncate_children(2);
            assert_eq!(Tree::get_child_count(root), 2);
            assert!(ptr::eq(root.get_child(1).unwrap(), leaves[1]));
            assert!(root.get_child(2).is_none());
        });
    }
}
