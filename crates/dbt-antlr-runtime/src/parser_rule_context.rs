//! Full parser node
use std::any::type_name;
use std::borrow::{Borrow, BorrowMut};
use std::fmt::{Debug, Error, Formatter};
use std::iter::FusedIterator;
use std::ops::{Deref, DerefMut};

use crate::errors::ANTLRError;
use crate::rule_context::{
    BaseRuleContext, CustomRuleContext, EmptyCustomRuleContext, EmptyNodeKind, RuleContext,
};
use crate::token::{CommonToken, Token};
use crate::tree::{
    ChildrenOfType, CtxPtr, NodeChildren, NodeInner, NodeKindType, TerminalNode, TokensOfType,
    Tree as _, TreeChildren, TreeNode,
};
use crate::{token_factory, Arena};

/// Language-agnostic behaviors of the Antlr AST.
///
/// This is the language-agnostic, dyn-compatible interface for parser rule
/// contexts.
pub trait ParserRuleContext<'input, 'arena>: RuleContext<'arena> + Debug
where
    'input: 'arena,
{
    /// Get the initial token in this context.
    ///
    /// Note that the range from start to stop is inclusive, so for rules that do not consume anything
    /// (for example, zero length or error productions) this token may exceed stop.
    fn start(&self) -> &'arena dyn Token {
        unimplemented!()
    }

    /// Get the final token in this context.
    ///
    /// Note that the range from start to stop is inclusive, so for rules that do not consume anything
    /// (for example, zero length or error productions) this token may precede start.
    fn stop(&self) -> &'arena dyn Token {
        unimplemented!()
    }

    fn get_parent_ctx(&self) -> Option<&'arena dyn ParserRuleContext<'input, 'arena>> {
        None
    }

    fn get_child_ctx(&self, _i: usize) -> Option<&'arena dyn ParserRuleContext<'input, 'arena>> {
        None
    }

    fn get_child_count(&self) -> usize;

    fn iter_children<'a>(&'a self) -> ChildrenIter<'a, 'input, 'arena>
    where
        'input: 'a,
        'arena: 'a;

    fn get_token(&self, _ttype: i32, _pos: usize) -> Option<&'arena dyn Token>;

    fn get_tokens<'a>(&'a self, _ttype: i32) -> TokensIter<'a, 'input, 'arena>
    where
        'input: 'a,
        'arena: 'a;

    /// Returns the underlying token if this node is a terminal node.
    #[inline]
    fn as_terminal_token(&self) -> Option<&'arena dyn Token> {
        None
    }

    /// Return combined text of this AST node.
    /// To create resulting string it does traverse whole subtree,
    /// also it includes only tokens added to the parse tree
    ///
    /// Since tokens on hidden channels (e.g. whitespace or comments) are not
    /// added to the parse trees, they will not appear in the output of this
    /// method.
    fn get_text(&self) -> String;

    /// Print out a whole tree, not just a node, in LISP format `(root child1 ..
    /// childN)`. Print just a node if this is a leaf.
    fn to_string_tree(&self, rule_names: &[&str]) -> String {
        crate::trees::string_tree(self, rule_names)
    }
}

pub type EmptyParserRuleContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<
    'input,
    'arena,
    EmptyCustomRuleContext<'input, 'arena>,
    EmptyNodeKind,
    Tok,
>;

/// Stops the compilation if the alignment of `Ext` is larger than the
/// alignment of `usize`.
///
/// The context of a rule node starts at `TreeNode::body`, and it must have the
/// exact same layout as `ParserRuleContextCommon`. An `Ext` with a larger
/// alignment adds padding which moves the context and its fields away from
/// these pre-determined offsets.
///
/// ```
/// use dbt_antlr_runtime::parser_rule_context::assert_ext_layout;
///
/// const _: () = assert_ext_layout::<(u64, Option<&str>)>();
/// ```
///
/// ```compile_fail,E0080
/// use dbt_antlr_runtime::parser_rule_context::assert_ext_layout;
///
/// #[repr(align(16))]
/// struct OverAligned(u8);
///
/// const _: () = assert_ext_layout::<OverAligned>();
/// ```
#[doc(hidden)]
pub const fn assert_ext_layout<Ext>() {
    assert!(
        align_of::<Ext>() <= align_of::<usize>(),
        "the alignment of a rule context extension type must not be larger than the alignment of usize"
    );
}

/// Core AST node type -- this augments [BaseParserRuleContext] with
/// additional states that allows it to be strung together into a tree, as well
/// as tying it back to the corresponding input.
///
/// This is Rust's version of the `ParserRuleContext` "abstract base class", it
/// will be specialized into language-specific concrete types by monomorphizing
/// the `Ext` type parameter, which is implemented by generated code.
#[repr(C)]
pub struct BaseParserRuleContext<'input, 'arena, Ext, NodeKind, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
    NodeKind: NodeKindType<'arena, Tok>,
    Ext: CustomRuleContext<'input, 'arena, Tok, NodeKind = NodeKind>,
{
    pub(crate) start: Option<&'arena Tok>,
    pub(crate) stop: Option<&'arena Tok>,
    pub(crate) children: NodeChildren<'input, 'arena, NodeKind, Tok>,

    pub(crate) base: BaseRuleContext<'input, 'arena, Ext, NodeKind, Tok>,

    // TODO: figure out what on earth this is used for and whether we can get
    // rid of it
    exception: (),
}

impl<'input, 'arena, Ext, NodeKind, Tok> Debug
    for BaseParserRuleContext<'input, 'arena, Ext, NodeKind, Tok>
where
    'input: 'arena,
    Tok: Token,
    NodeKind: NodeKindType<'arena, Tok>,
    Ext: CustomRuleContext<'input, 'arena, Tok, NodeKind = NodeKind>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str(type_name::<Self>())
    }
}

impl<'input, 'arena, Ext, NodeKind, Tok> RuleContext<'arena>
    for BaseParserRuleContext<'input, 'arena, Ext, NodeKind, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
    NodeKind: NodeKindType<'arena, Tok>,
    Ext: CustomRuleContext<'input, 'arena, Tok, NodeKind = NodeKind> + 'arena,
{
    fn get_invoking_state(&self) -> i32 {
        self.as_node().get_invoking_state()
    }

    fn get_parent_ctx(&self) -> Option<&'arena dyn RuleContext<'arena>> {
        self.base
            .parent()
            .map(move |rc| rc as &dyn RuleContext<'arena>)
    }

    fn get_rule_index(&self) -> usize {
        self.base.get_rule_index()
    }

    fn get_alt_number(&self) -> i32 {
        self.base.get_alt_number()
    }

    fn get_node_text(&self, rule_names: &[&str]) -> String {
        self.base.get_node_text(rule_names)
    }
}

impl<'input, 'arena, Ext, NodeKind, Tok> Deref
    for BaseParserRuleContext<'input, 'arena, Ext, NodeKind, Tok>
where
    'input: 'arena,
    Tok: Token,
    NodeKind: NodeKindType<'arena, Tok>,
    Ext: CustomRuleContext<'input, 'arena, Tok, NodeKind = NodeKind>,
{
    type Target = Ext;
    fn deref(&self) -> &Self::Target {
        &self.base.ext
    }
}

impl<'input, 'arena, Ext, NodeKind, Tok> DerefMut
    for BaseParserRuleContext<'input, 'arena, Ext, NodeKind, Tok>
where
    'input: 'arena,
    Tok: Token,
    NodeKind: NodeKindType<'arena, Tok>,
    Ext: CustomRuleContext<'input, 'arena, Tok, NodeKind = NodeKind>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base.ext
    }
}

impl<'input, 'arena, Ext, NodeKind, Tok> Borrow<Ext>
    for BaseParserRuleContext<'input, 'arena, Ext, NodeKind, Tok>
where
    'input: 'arena,
    Tok: Token,
    NodeKind: NodeKindType<'arena, Tok>,
    Ext: CustomRuleContext<'input, 'arena, Tok, NodeKind = NodeKind>,
{
    fn borrow(&self) -> &Ext {
        &self.base.ext
    }
}

impl<'input, 'arena, Ext, NodeKind, Tok> BorrowMut<Ext>
    for BaseParserRuleContext<'input, 'arena, Ext, NodeKind, Tok>
where
    'input: 'arena,
    Tok: Token,
    NodeKind: NodeKindType<'arena, Tok>,
    Ext: CustomRuleContext<'input, 'arena, Tok, NodeKind = NodeKind>,
{
    fn borrow_mut(&mut self) -> &mut Ext {
        &mut self.base.ext
    }
}

impl<'input, 'arena, Ext, NodeKind, Tok> ParserRuleContext<'input, 'arena>
    for BaseParserRuleContext<'input, 'arena, Ext, NodeKind, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
    NodeKind: NodeKindType<'arena, Tok>,
    Ext: CustomRuleContext<'input, 'arena, Tok, NodeKind = NodeKind> + 'arena,
{
    #[inline]
    fn start(&self) -> &'arena dyn Token {
        self.start
            .map_or_else(|| token_factory::invalid() as _, |t| t as _)
    }

    #[inline]
    fn stop(&self) -> &'arena dyn Token {
        self.stop
            .map_or_else(|| token_factory::invalid() as _, |t| t as _)
    }

    fn get_parent_ctx(&self) -> Option<&'arena dyn ParserRuleContext<'input, 'arena>> {
        self.base
            .parent()
            .map(move |rc| rc as &dyn ParserRuleContext<'input, 'arena>)
    }

    fn get_child_ctx(&self, i: usize) -> Option<&'arena dyn ParserRuleContext<'input, 'arena>> {
        self.children
            .get(i)
            .map(|item| item as &dyn ParserRuleContext<'input, 'arena>)
    }

    fn get_child_count(&self) -> usize {
        self.children.len()
    }

    fn iter_children<'a>(&'a self) -> ChildrenIter<'a, 'input, 'arena>
    where
        'input: 'a,
        'arena: 'a,
    {
        ChildrenIter::new(self)
    }

    fn get_token(&self, ttype: i32, pos: usize) -> Option<&'arena dyn Token> {
        self.token_of_type(ttype, pos)
            .map(|t| t.symbol as &'arena dyn Token)
    }

    fn get_tokens<'a>(&'a self, ttype: i32) -> TokensIter<'a, 'input, 'arena>
    where
        'input: 'a,
        'arena: 'a,
    {
        TokensIter::new(self, ttype)
    }

    fn get_text(&self) -> String {
        let mut result = String::new();

        for child in self.children.iter() {
            result += &ParserRuleContext::get_text(*child)
        }

        result
    }
}

impl<'input, 'arena, Ctx, Node, Tok> NodeInner<'input, 'arena, Node, Tok>
    for BaseParserRuleContext<'input, 'arena, Ctx, Node, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
    Node: NodeKindType<'arena, Tok>,
    Ctx: CustomRuleContext<'input, 'arena, Tok, NodeKind = Node> + 'arena,
{
    fn cast_from<'a>(node: &'a TreeNode<'input, 'arena, Node, Tok>) -> Option<&'a Self> {
        Ctx::cast_from(node)
    }

    #[inline(always)]
    unsafe fn cast_from_unchecked<'a>(node: &'a TreeNode<'input, 'arena, Node, Tok>) -> &'a Self {
        Ctx::cast_from_unchecked(node)
    }

    fn cast_from_mut<'a>(
        node: &'a mut TreeNode<'input, 'arena, Node, Tok>,
    ) -> Option<&'a mut Self> {
        Ctx::cast_from_mut(node)
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

    #[inline]
    fn iter_child_nodes(&self) -> TreeChildren<'_, 'arena, TreeNode<'input, 'arena, Node, Tok>> {
        self.get_children()
    }
}

#[allow(missing_docs)]
impl<'input, 'arena, Ext, Node, Tok> BaseParserRuleContext<'input, 'arena, Ext, Node, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
    Node: NodeKindType<'arena, Tok>,
    Ext: CustomRuleContext<'input, 'arena, Tok, NodeKind = Node> + 'arena,
{
    /// Construct a new parser rule context with the given parent, invoking
    /// state, and extension data.
    #[inline]
    pub fn create(
        arena: &'arena Arena,
        parent: Option<&'arena TreeNode<'input, 'arena, Ext::NodeKind, Tok>>,
        invoking_state: i32,
        ext: Ext,
    ) -> Result<&'arena mut TreeNode<'input, 'arena, Ext::NodeKind, Tok>, ANTLRError> {
        const { assert_ext_layout::<Ext>() };
        let ctx = Self {
            start: None,
            stop: None,
            children: NodeChildren::new(),
            base: BaseRuleContext::new(parent, ext),
            exception: (),
        };
        let node = Ext::make_node(arena, ctx);
        if node.is_null() {
            #[cfg(feature = "arena-allocation-limit")]
            {
                return Err(ANTLRError::arena_allocation_limit_exceeded(
                    arena.allocation_limit_bytes(),
                    arena.total_allocated_bytes(),
                ));
            }
            #[cfg(not(feature = "arena-allocation-limit"))]
            {
                std::alloc::handle_alloc_error(std::alloc::Layout::new::<
                    TreeNode<'input, 'arena, Ext::NodeKind, Tok>,
                >())
            }
        }
        // SAFETY: The node is allocated by the arena and lives for 'arena, and the
        // header is properly initialized by make_node.
        let node = unsafe { &mut *node };
        node.node_tag = Ext::node_tag();
        node.set_invoking_state(invoking_state);
        Ok(node)
    }

    pub fn morph<Tgt>(
        self,
        ctor: impl FnOnce(Ext) -> Tgt,
    ) -> BaseParserRuleContext<'input, 'arena, Tgt, Node, Tok>
    where
        Tgt: CustomRuleContext<'input, 'arena, Tok, NodeKind = Node>,
    {
        const { assert_ext_layout::<Tgt>() };
        BaseParserRuleContext {
            base: self.base.morph(ctor),
            start: self.start,
            stop: self.stop,
            exception: self.exception,
            children: self.children,
        }
    }

    pub fn get_parent(&self) -> Option<&'arena TreeNode<'input, 'arena, Ext::NodeKind, Tok>> {
        self.base.parent()
    }

    pub fn has_parent(&self) -> bool {
        self.base.has_parent()
    }

    pub fn set_parent(
        &mut self,
        parent: Option<&'arena TreeNode<'input, 'arena, Ext::NodeKind, Tok>>,
    ) {
        self.base.set_parent(parent);
    }

    pub fn set_exception(&self, _e: ANTLRError, _arena: &'arena Arena) {
        // alloc returns &mut T from the bump arena; converting to NonNull is
        // always non-null and the allocation lives for 'arena.
        // let ptr = NonNull::from(arena.alloc_payload(e));
        // self.exception.set(Some(ptr));
    }

    pub fn set_invoking_state(&mut self, t: i32) {
        self.as_node_mut().set_invoking_state(t);
    }

    pub fn set_alt_number(&mut self, _alt_number: i32) {
        self.base.set_alt_number(_alt_number)
    }

    pub fn set_start(&mut self, t: Option<&'arena Tok>) {
        self.start = t;
    }

    pub fn set_stop(&mut self, t: Option<&'arena Tok>) {
        self.stop = t;
    }

    pub fn remove_last_child(&mut self) {
        self.children.pop();
    }

    pub fn add_child(
        &mut self,
        child: &'arena TreeNode<'input, 'arena, Ext::NodeKind, Tok>,
        arena: &'arena Arena,
    ) {
        self.children.push(child, arena);
    }

    pub fn get_child(
        &self,
        i: usize,
    ) -> Option<&'arena TreeNode<'input, 'arena, Ext::NodeKind, Tok>> {
        self.children.get(i)
    }

    #[inline]
    pub fn get_children(
        &self,
    ) -> TreeChildren<'_, 'arena, TreeNode<'input, 'arena, Ext::NodeKind, Tok>> {
        TreeChildren::new(self.children.as_slice())
    }

    /// Return the direct children of this rule context as a slice.
    #[inline]
    pub fn children_slice(&self) -> &[&'arena TreeNode<'input, 'arena, Ext::NodeKind, Tok>] {
        self.children.as_slice()
    }

    /// Returns the `pos`-th direct child node that matches rule context type `T`.
    #[inline]
    pub fn child_of_type<T>(&self, pos: usize) -> Option<&'arena T>
    where
        T: ParserRuleContext<'input, 'arena> + NodeInner<'input, 'arena, Node, Tok>,
    {
        self.children_of_type::<T>().nth(pos)
    }

    /// Returns an iterator over direct child nodes that match rule context type `T`.
    #[inline]
    pub fn children_of_type<T>(&self) -> ChildrenOfType<'_, 'input, 'arena, Node, Tok, T>
    where
        T: ParserRuleContext<'input, 'arena> + NodeInner<'input, 'arena, Node, Tok>,
    {
        ChildrenOfType::new(self.children.as_slice())
    }

    /// Returns the `pos`-th direct [`TerminalNode`] child with token type `ttype`.
    #[inline]
    pub fn token_of_type(
        &self,
        ttype: i32,
        pos: usize,
    ) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.tokens_of_type(ttype).nth(pos)
    }

    /// Returns an iterator over direct [`TerminalNode`] children with token type `ttype`.
    #[inline]
    pub fn tokens_of_type(&self, ttype: i32) -> TokensOfType<'_, 'input, 'arena, Node, Tok> {
        TokensOfType::new(self.children.as_slice(), ttype)
    }

    // pub fn to_string(self: Rc<Self>, rule_names: Option<&[&str]>, stop: Option<Rc<Ctx::Ctx::Type>>) -> String {
    //     (self as Rc<dyn ParserRuleContext>).to_string(rule_names, stop)
    // }

    /// Prints list of parent rules
    pub fn to_string(
        &'arena self,
        rule_names: Option<&[&str]>,
        stop: Option<&'arena TreeNode<'input, 'arena, Ext::NodeKind, Tok>>,
    ) -> String {
        let mut result = String::from("[");
        let mut next = Some(self.as_node());
        while let Some(p) = next {
            if stop.is_some_and(|s| std::ptr::eq(s, p)) {
                break;
            }

            if let Some(rule_names) = rule_names {
                let rule_index = p.get_rule_index();
                let rule_name = rule_names
                    .get(rule_index)
                    .map_or_else(|| rule_index.to_string(), |&it| it.to_owned());
                result.push_str(&rule_name);
                result.push(' ');
            } else if !p.is_empty() {
                result.push_str(&p.get_invoking_state().to_string());
                result.push(' ');
            }

            next = p.get_parent();
        }

        if result.ends_with(' ') {
            result.pop();
        }

        result.push(']');
        result
    }
}

/// Iterator over direct children of a [`ParserRuleContext`].
pub struct ChildrenIter<'a, 'input, 'arena>
where
    'input: 'arena,
{
    ctx: &'a dyn ParserRuleContext<'input, 'arena>,
    front: usize,
    back: usize,
}

impl<'a, 'input, 'arena> ChildrenIter<'a, 'input, 'arena>
where
    'input: 'arena,
{
    #[inline]
    pub(crate) fn new(ctx: &'a dyn ParserRuleContext<'input, 'arena>) -> Self {
        let back = ctx.get_child_count();
        Self {
            ctx,
            front: 0,
            back,
        }
    }
}

impl<'input, 'arena> Clone for ChildrenIter<'_, 'input, 'arena>
where
    'input: 'arena,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx,
            front: self.front,
            back: self.back,
        }
    }
}

impl<'input, 'arena> Debug for ChildrenIter<'_, 'input, 'arena>
where
    'input: 'arena,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("ChildrenIter")
            .field("front", &self.front)
            .field("back", &self.back)
            .finish()
    }
}

impl<'input, 'arena> Iterator for ChildrenIter<'_, 'input, 'arena>
where
    'input: 'arena,
{
    type Item = &'arena dyn ParserRuleContext<'input, 'arena>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            let idx = self.front;
            self.front += 1;
            let item = self.ctx.get_child_ctx(idx);
            if item.is_none() {
                self.front = self.back;
            }
            item
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.back - self.front;
        (rem, Some(rem))
    }

    #[inline]
    fn count(self) -> usize {
        self.back - self.front
    }

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let rem = self.back - self.front;
        if n < rem {
            let idx = self.front + n;
            self.front = idx + 1;
            let item = self.ctx.get_child_ctx(idx);
            if item.is_none() {
                self.front = self.back;
            }
            item
        } else {
            self.front = self.back;
            None
        }
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'input, 'arena> DoubleEndedIterator for ChildrenIter<'_, 'input, 'arena>
where
    'input: 'arena,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front < self.back {
            self.back -= 1;
            let item = self.ctx.get_child_ctx(self.back);
            if item.is_none() {
                self.back = self.front;
            }
            item
        } else {
            None
        }
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let rem = self.back - self.front;
        if n < rem {
            self.back -= n + 1;
            let item = self.ctx.get_child_ctx(self.back);
            if item.is_none() {
                self.back = self.front;
            }
            item
        } else {
            self.back = self.front;
            None
        }
    }
}

impl<'input, 'arena> ExactSizeIterator for ChildrenIter<'_, 'input, 'arena>
where
    'input: 'arena,
{
    #[inline]
    fn len(&self) -> usize {
        self.back - self.front
    }
}

impl<'input, 'arena> FusedIterator for ChildrenIter<'_, 'input, 'arena> where 'input: 'arena {}

/// Iterator over direct terminal tokens of a [`ParserRuleContext`] that match a token type.
pub struct TokensIter<'a, 'input, 'arena>
where
    'input: 'arena,
{
    children: ChildrenIter<'a, 'input, 'arena>,
    ttype: i32,
}

impl<'a, 'input, 'arena> TokensIter<'a, 'input, 'arena>
where
    'input: 'arena,
{
    #[inline]
    pub(crate) fn new(ctx: &'a dyn ParserRuleContext<'input, 'arena>, ttype: i32) -> Self {
        Self {
            children: ChildrenIter::new(ctx),
            ttype,
        }
    }
}

impl<'input, 'arena> Clone for TokensIter<'_, 'input, 'arena>
where
    'input: 'arena,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            children: self.children.clone(),
            ttype: self.ttype,
        }
    }
}

impl<'input, 'arena> Debug for TokensIter<'_, 'input, 'arena>
where
    'input: 'arena,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("TokensIter")
            .field("ttype", &self.ttype)
            .field("remaining_children", &self.children.len())
            .finish()
    }
}

impl<'input, 'arena> Iterator for TokensIter<'_, 'input, 'arena>
where
    'input: 'arena,
{
    type Item = &'arena dyn Token;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        for child in self.children.by_ref() {
            if let Some(tok) = child.as_terminal_token() {
                if tok.get_token_type() == self.ttype {
                    return Some(tok);
                }
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.children.len()))
    }
}

impl<'input, 'arena> DoubleEndedIterator for TokensIter<'_, 'input, 'arena>
where
    'input: 'arena,
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(child) = self.children.next_back() {
            if let Some(tok) = child.as_terminal_token() {
                if tok.get_token_type() == self.ttype {
                    return Some(tok);
                }
            }
        }
        None
    }
}

impl<'input, 'arena> FusedIterator for TokensIter<'_, 'input, 'arena> where 'input: 'arena {}
