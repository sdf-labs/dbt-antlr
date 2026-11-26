//! `#[repr(C)]` mirrors of the runtime parse tree layout.
//!
//! The types in this module describe arena memory that `dbt-antlr-runtime`
//! owns. C code reads parse trees through pointers to these types: it never
//! copies them and it never writes through them. The compile-time assertions
//! at the end of this module make sure that the mirrors agree with
//! [`dbt_antlr_runtime::layout`].
//!
//! A leaf node is only [`ANTLR_NODE_LEAF_SIZE`] bytes long. Do not read an
//! [`AntlrTreeNode`] by value. Examine `node_tag` first, and then read only
//! the fields of the active [`AntlrNodeBody`] member.

use std::ffi::c_void;
use std::mem::{align_of, offset_of, size_of};

use dbt_antlr_runtime::layout;

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
    /// source; get their text with `antlr_token_text_fallback`.
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
    assert!(ANTLR_NODE_TAG_ERROR == layout::NODE_TAG_ERROR);
    assert!(ANTLR_NODE_TAG_TERMINAL == layout::NODE_TAG_TERMINAL);
    assert!(ANTLR_NODE_TAG_FIRST_RULE == layout::NODE_TAG_FIRST_RULE);
    assert!(ANTLR_CHILDREN_INLINE_SLOTS == layout::CHILDREN_INLINE_SLOTS);
    assert!(ANTLR_TOKEN_TYPE_BITS == layout::TOKEN_TYPE_BITS);
    assert!(ANTLR_TOKEN_EOF == dbt_antlr_runtime::token::TOKEN_EOF);

    assert!(size_of::<AntlrToken>() == layout::TOKEN_SIZE);
    assert!(align_of::<AntlrToken>() == layout::TOKEN_ALIGN);
    assert!(offset_of!(AntlrToken, source) == layout::TOKEN_SOURCE_OFFSET);
    assert!(offset_of!(AntlrToken, type_and_channel) == layout::TOKEN_TYPE_AND_CHANNEL_OFFSET);
    assert!(offset_of!(AntlrToken, start) == layout::TOKEN_START_OFFSET);
    assert!(offset_of!(AntlrToken, stop) == layout::TOKEN_STOP_OFFSET);
    assert!(offset_of!(AntlrToken, token_index) == layout::TOKEN_INDEX_OFFSET);

    assert!(align_of::<AntlrTreeNode>() == layout::NODE_ALIGN);
    assert!(offset_of!(AntlrTreeNode, label_tag) == layout::NODE_LABEL_TAG_OFFSET);
    assert!(offset_of!(AntlrTreeNode, node_tag) == layout::NODE_TAG_OFFSET);
    assert!(offset_of!(AntlrTreeNode, invoking_state) == layout::NODE_INVOKING_STATE_OFFSET);
    assert!(offset_of!(AntlrTreeNode, body) == layout::NODE_BODY_OFFSET);
    assert!(offset_of!(AntlrTreeNode, body) == layout::NODE_HEADER_SIZE);

    assert!(size_of::<AntlrRuleBody>() == layout::RULE_BODY_SIZE);
    assert!(offset_of!(AntlrRuleBody, start) == layout::RULE_BODY_START_OFFSET);
    assert!(offset_of!(AntlrRuleBody, stop) == layout::RULE_BODY_STOP_OFFSET);
    assert!(offset_of!(AntlrRuleBody, children) == layout::RULE_BODY_CHILDREN_OFFSET);
    assert!(offset_of!(AntlrRuleBody, parent) == layout::RULE_BODY_PARENT_OFFSET);

    assert!(size_of::<AntlrNodeChildren>() == layout::CHILDREN_SIZE);
    assert!(offset_of!(AntlrNodeChildren, len) == layout::CHILDREN_LEN_OFFSET);
    assert!(offset_of!(AntlrNodeChildren, cap) == layout::CHILDREN_CAP_OFFSET);
    assert!(offset_of!(AntlrNodeChildren, storage) == layout::CHILDREN_STORAGE_OFFSET);
    assert!(size_of::<AntlrChildrenStore>() == size_of::<[*const AntlrTreeNode; 3]>());

    assert!(size_of::<AntlrLeafBody>() == layout::LEAF_BODY_SIZE);
    assert!(offset_of!(AntlrLeafBody, symbol) == layout::LEAF_BODY_SYMBOL_OFFSET);
    assert!(ANTLR_NODE_LEAF_SIZE == layout::NODE_HEADER_SIZE + layout::LEAF_BODY_SIZE);
};
