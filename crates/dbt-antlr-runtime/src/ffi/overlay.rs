//! `#[repr(C)]` mirrors of the runtime parse tree layout.
//!
//! The types in this module describe arena memory that `dbt-antlr-runtime`
//! owns. C code reads parse trees through pointers to these types: it never
//! copies them and it never writes through them. The compile-time assertions
//! at the end of this module make sure that the mirrors agree with the
//! runtime structs.
//!
//! A leaf node is only [`ANTLR_NODE_LEAF_SIZE`] bytes long. Do not read an
//! [`AntlrTreeNode`] by value. Examine `node_tag` first, and then read only
//! the fields of the active [`AntlrNodeBody`] member.

use std::ffi::c_void;
use std::mem::{align_of, offset_of, size_of};

/// The version of the C ABI. A native client compares this value with the
/// return value of `antlr_abi_version` at load time.
pub const ANTLR_ABI_VERSION: u32 = 1;
/// The `node_tag` of an error leaf.
pub const ANTLR_NODE_TAG_ERROR: u16 = 0;
/// The `node_tag` of a terminal leaf.
pub const ANTLR_NODE_TAG_TERMINAL: u16 = 1;
/// The `node_tag` of the rule with index 0. A rule node with tag `t` has the
/// rule index `t - ANTLR_NODE_TAG_FIRST_RULE`.
pub const ANTLR_NODE_TAG_FIRST_RULE: u16 = 2;
/// The number of bytes that the runtime allocates for a leaf node.
pub const ANTLR_NODE_LEAF_SIZE: usize = 16;
/// The number of child pointers that a rule node holds inline.
pub const ANTLR_CHILDREN_INLINE_SLOTS: usize = 3;
/// The number of low bits of `AntlrToken::type_and_channel` that hold the
/// signed token type. The high bits hold the signed channel.
pub const ANTLR_TOKEN_TYPE_BITS: u32 = 22;
/// The token type of the EOF token.
pub const ANTLR_TOKEN_EOF: i32 = -1;

/// A token of the parse tree (the runtime type `CommonToken`).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntlrToken {
    /// The identity of the source that holds the token text. All tokens that
    /// slice the input have the `primary_source` of their session. Other
    /// tokens (for example conjured `<missing ...>` tokens) have a different
    /// source. Get their text with `antlr_token_text_fallback`.
    pub source: *const c_void,
    /// The low `ANTLR_TOKEN_TYPE_BITS` bits hold the signed token type, and
    /// the high bits hold the signed channel. Use `antlr_token_type` and
    /// `antlr_token_channel` to decode them.
    pub type_and_channel: i32,
    /// The UTF-8 byte offset of the first byte of the token, or -1 for a
    /// conjured token.
    pub start: i32,
    /// The UTF-8 byte offset of the last byte of the token (inclusive), or -1
    /// for a conjured token. For an empty token, `stop` is `start - 1`.
    pub stop: i32,
    /// The index of the token in the token stream, or -1 for a conjured
    /// token.
    pub token_index: i32,
}

/// The storage of the children of a rule node.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntlrChildrenStore {
    /// The children, when `AntlrNodeChildren::cap` is 0.
    pub inline_slots: [*const AntlrTreeNode; 3],
    /// An array of `AntlrNodeChildren::cap` child pointers, when
    /// `AntlrNodeChildren::cap` is not 0.
    pub spilled: *const *const AntlrTreeNode,
}

/// The children of a rule node.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntlrNodeChildren {
    /// The number of children.
    pub len: u32,
    /// 0 when the children are in `storage.inline_slots`. Otherwise, the
    /// capacity of the `storage.spilled` array.
    pub cap: u32,
    /// The child pointers.
    pub storage: AntlrChildrenStore,
}

/// The body of a rule node.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntlrRuleBody {
    /// The first token of the rule, or NULL.
    pub start: *const AntlrToken,
    /// The last token of the rule, or NULL. When the rule matched no token,
    /// `stop` comes before `start` in the token stream.
    pub stop: *const AntlrToken,
    /// The children of the rule node.
    pub children: AntlrNodeChildren,
    /// The parent rule node, or NULL for the root.
    pub parent: *const AntlrTreeNode,
}

/// The body of an error leaf or a terminal leaf.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntlrLeafBody {
    /// The token of the leaf. It is never NULL.
    pub symbol: *const AntlrToken,
}

/// The body of a node. `AntlrTreeNode::node_tag` selects the active member.
#[repr(C)]
#[derive(Clone, Copy)]
pub union AntlrNodeBody {
    /// The active member when `node_tag >= ANTLR_NODE_TAG_FIRST_RULE`.
    pub rule: AntlrRuleBody,
    /// The active member when `node_tag < ANTLR_NODE_TAG_FIRST_RULE`.
    pub leaf: AntlrLeafBody,
}

impl std::fmt::Debug for AntlrChildrenStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AntlrChildrenStore").finish_non_exhaustive()
    }
}

impl std::fmt::Debug for AntlrNodeBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AntlrNodeBody").finish_non_exhaustive()
    }
}

/// A node of a parse tree.
///
/// Leaf nodes are only `ANTLR_NODE_LEAF_SIZE` bytes long. Never copy a node
/// by value: read only the fields of the body member that `node_tag`
/// selects.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntlrTreeNode {
    /// For a rule with labeled alternatives (`# label`), the index of the
    /// label in `AntlrRuleInfo::alt_labels`. The value
    /// `AntlrRuleInfo::alt_label_count` tells that no label applies. For other
    /// nodes, the value is 0.
    pub label_tag: u16,
    /// `ANTLR_NODE_TAG_ERROR`, `ANTLR_NODE_TAG_TERMINAL`, or
    /// `ANTLR_NODE_TAG_FIRST_RULE` plus the rule index.
    pub node_tag: u16,
    /// The ATN state that invoked the rule, or -1.
    pub invoking_state: i32,
    /// The rule body or the leaf body.
    pub body: AntlrNodeBody,
}

const _: () = {
    use crate::rule_context::EmptyNodeKind;
    use crate::token::{CommonToken, TOKEN_EOF, TOKEN_TYPE_BITS};
    use crate::tree::{
        NodeChildren, ParserRuleContextCommon, TerminalNode, TreeNode, MAX_INLINE_CHILDREN,
    };

    type Tok = CommonToken<'static>;
    type Node = TreeNode<'static, 'static, EmptyNodeKind>;
    type Common = ParserRuleContextCommon<'static, 'static, EmptyNodeKind>;
    type Children = NodeChildren<'static, 'static, EmptyNodeKind>;
    type Leaf = TerminalNode<'static, 'static>;

    assert!(ANTLR_NODE_TAG_ERROR == EmptyNodeKind::Error as u16);
    assert!(ANTLR_NODE_TAG_TERMINAL == EmptyNodeKind::Terminal as u16);
    assert!(ANTLR_NODE_TAG_FIRST_RULE == EmptyNodeKind::EmptyContext as u16);
    assert!(ANTLR_CHILDREN_INLINE_SLOTS == MAX_INLINE_CHILDREN);
    assert!(ANTLR_TOKEN_TYPE_BITS == TOKEN_TYPE_BITS);
    assert!(ANTLR_TOKEN_EOF == TOKEN_EOF);

    assert!(size_of::<AntlrToken>() == size_of::<Tok>());
    assert!(align_of::<AntlrToken>() == align_of::<Tok>());
    assert!(offset_of!(AntlrToken, source) == offset_of!(Tok, source));
    assert!(offset_of!(AntlrToken, type_and_channel) == offset_of!(Tok, type_and_channel));
    assert!(offset_of!(AntlrToken, start) == offset_of!(Tok, start));
    assert!(offset_of!(AntlrToken, stop) == offset_of!(Tok, stop));
    assert!(offset_of!(AntlrToken, token_index) == offset_of!(Tok, token_index));

    assert!(align_of::<AntlrTreeNode>() == align_of::<Node>());
    assert!(offset_of!(AntlrTreeNode, label_tag) == offset_of!(Node, label_tag));
    assert!(offset_of!(AntlrTreeNode, node_tag) == offset_of!(Node, node_tag));
    assert!(offset_of!(AntlrTreeNode, invoking_state) == offset_of!(Node, invoking_state));
    assert!(offset_of!(AntlrTreeNode, body) == offset_of!(Node, body));
    assert!(offset_of!(AntlrTreeNode, body) == size_of::<Node>());

    assert!(size_of::<AntlrRuleBody>() == size_of::<Common>());
    assert!(offset_of!(AntlrRuleBody, start) == offset_of!(Common, start));
    assert!(offset_of!(AntlrRuleBody, stop) == offset_of!(Common, stop));
    assert!(offset_of!(AntlrRuleBody, children) == offset_of!(Common, children));
    assert!(offset_of!(AntlrRuleBody, parent) == offset_of!(Common, parent));

    assert!(size_of::<AntlrNodeChildren>() == size_of::<Children>());
    assert!(offset_of!(AntlrNodeChildren, len) == offset_of!(Children, len));
    assert!(offset_of!(AntlrNodeChildren, cap) == offset_of!(Children, cap));
    assert!(offset_of!(AntlrNodeChildren, storage) == offset_of!(Children, storage));
    assert!(size_of::<AntlrChildrenStore>() == size_of::<[*const AntlrTreeNode; 3]>());

    assert!(size_of::<AntlrLeafBody>() == size_of::<Leaf>());
    assert!(offset_of!(AntlrLeafBody, symbol) == offset_of!(Leaf, symbol));
    assert!(ANTLR_NODE_LEAF_SIZE == size_of::<Node>() + size_of::<Leaf>());
};
