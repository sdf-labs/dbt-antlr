use std::hash::Hash;

use crate::lexer::Lexer;

pub(crate) const LEXER_ACTION_TYPE_CHANNEL: i32 = 0;
pub(crate) const LEXER_ACTION_TYPE_CUSTOM: i32 = 1;
pub(crate) const LEXER_ACTION_TYPE_MODE: i32 = 2;
pub(crate) const LEXER_ACTION_TYPE_MORE: i32 = 3;
pub(crate) const LEXER_ACTION_TYPE_POP_MODE: i32 = 4;
pub(crate) const LEXER_ACTION_TYPE_PUSH_MODE: i32 = 5;
pub(crate) const LEXER_ACTION_TYPE_SKIP: i32 = 6;
pub(crate) const LEXER_ACTION_TYPE_TYPE: i32 = 7;

/// One lexer command attached to an action transition.
///
/// Every variant except the runtime-only `IndexedCustom` maps onto an entry of
/// the serialized ATN action table.
#[allow(clippy::enum_variant_names)]
#[repr(C, u32)]
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash)]
pub enum LexerAction {
    Channel(i32) = LEXER_ACTION_TYPE_CHANNEL as u32,
    Custom {
        rule_index: i32,
        action_index: i32,
    } = LEXER_ACTION_TYPE_CUSTOM as u32,
    Mode(i32) = LEXER_ACTION_TYPE_MODE as u32,
    More = LEXER_ACTION_TYPE_MORE as u32,
    PopMode = LEXER_ACTION_TYPE_POP_MODE as u32,
    PushMode(i32) = LEXER_ACTION_TYPE_PUSH_MODE as u32,
    #[default]
    Skip = LEXER_ACTION_TYPE_SKIP as u32,
    Type(i32) = LEXER_ACTION_TYPE_TYPE as u32,
    IndexedCustom {
        offset: i32,
        rule_index: i32,
        action_index: i32,
    } = 8,
}

impl LexerAction {
    /// The ATN `(type, data1, data2)` encoding of this action, the inverse
    /// of [`Self::from_serialized`].
    ///
    /// Panics on `IndexedCustom`: a position-fixed action has no ATN
    /// encoding (serialize the underlying custom action instead).
    pub fn to_serialized(&self) -> (i32, i32, i32) {
        match *self {
            LexerAction::Channel(channel) => (LEXER_ACTION_TYPE_CHANNEL, channel, 0),
            LexerAction::Custom {
                rule_index,
                action_index,
            } => (LEXER_ACTION_TYPE_CUSTOM, rule_index, action_index),
            LexerAction::Mode(mode) => (LEXER_ACTION_TYPE_MODE, mode, 0),
            LexerAction::More => (LEXER_ACTION_TYPE_MORE, 0, 0),
            LexerAction::PopMode => (LEXER_ACTION_TYPE_POP_MODE, 0, 0),
            LexerAction::PushMode(mode) => (LEXER_ACTION_TYPE_PUSH_MODE, mode, 0),
            LexerAction::Skip => (LEXER_ACTION_TYPE_SKIP, 0, 0),
            LexerAction::Type(ttype) => (LEXER_ACTION_TYPE_TYPE, ttype, 0),
            LexerAction::IndexedCustom { .. } => {
                unreachable!("position-fixed actions have no ATN encoding")
            }
        }
    }

    /// Rebuilds one action from its ATN `(type, data1, data2)` encoding.
    ///
    /// Panics on an unknown action type: the encoding is produced together
    /// with the ATN, so an invalid value is a build inconsistency, not a
    /// runtime condition.
    pub fn from_serialized(action_type: i32, data1: i32, data2: i32) -> LexerAction {
        match action_type {
            LEXER_ACTION_TYPE_CHANNEL => LexerAction::Channel(data1),
            LEXER_ACTION_TYPE_CUSTOM => LexerAction::Custom {
                rule_index: data1,
                action_index: data2,
            },
            LEXER_ACTION_TYPE_MODE => LexerAction::Mode(data1),
            LEXER_ACTION_TYPE_MORE => LexerAction::More,
            LEXER_ACTION_TYPE_POP_MODE => LexerAction::PopMode,
            LEXER_ACTION_TYPE_PUSH_MODE => LexerAction::PushMode(data1),
            LEXER_ACTION_TYPE_SKIP => LexerAction::Skip,
            LEXER_ACTION_TYPE_TYPE => LexerAction::Type(data1),
            _ => panic!("invalid lexer action type {action_type}"),
        }
    }

    pub fn is_position_dependent(&self) -> bool {
        matches!(
            self,
            LexerAction::Custom { .. } | LexerAction::IndexedCustom { .. }
        )
    }

    pub(crate) fn execute<'input, 'arena, Input, TF, L>(&self, lexer: &mut L)
    where
        'input: 'arena,
        L: Lexer<'input, 'arena, Input, TF>,
        Input: crate::char_stream::CharStream<'input>,
        TF: crate::token_factory::TokenFactory<'input, 'arena> + 'arena,
    {
        match *self {
            LexerAction::Channel(channel) => lexer.set_channel(channel),
            LexerAction::Custom {
                rule_index,
                action_index,
            }
            | LexerAction::IndexedCustom {
                rule_index,
                action_index,
                ..
            } => {
                lexer.action(None, rule_index, action_index);
            }
            LexerAction::Mode(mode) => lexer.set_mode(mode as usize),
            LexerAction::More => lexer.more(),
            LexerAction::PopMode => {
                lexer.pop_mode();
            }
            LexerAction::PushMode(mode) => lexer.push_mode(mode as usize),
            LexerAction::Skip => lexer.skip(),
            LexerAction::Type(ty) => lexer.set_type(ty),
        }
    }

    /// The 4-word `#[repr(C, u32)]` image representation of this action, with
    /// unused union payload words zero-filled so the image contains no
    /// uninitialized padding bytes.
    #[cfg(feature = "static-dfa-build")]
    pub(crate) fn to_image_words(self) -> [u32; 4] {
        match self {
            LexerAction::Channel(channel) => {
                [LEXER_ACTION_TYPE_CHANNEL as u32, channel as u32, 0, 0]
            }
            LexerAction::Custom {
                rule_index,
                action_index,
            } => [
                LEXER_ACTION_TYPE_CUSTOM as u32,
                rule_index as u32,
                action_index as u32,
                0,
            ],
            LexerAction::Mode(mode) => [LEXER_ACTION_TYPE_MODE as u32, mode as u32, 0, 0],
            LexerAction::More => [LEXER_ACTION_TYPE_MORE as u32, 0, 0, 0],
            LexerAction::PopMode => [LEXER_ACTION_TYPE_POP_MODE as u32, 0, 0, 0],
            LexerAction::PushMode(mode) => [LEXER_ACTION_TYPE_PUSH_MODE as u32, mode as u32, 0, 0],
            LexerAction::Skip => [LEXER_ACTION_TYPE_SKIP as u32, 0, 0, 0],
            LexerAction::Type(ttype) => [LEXER_ACTION_TYPE_TYPE as u32, ttype as u32, 0, 0],
            LexerAction::IndexedCustom {
                offset,
                rule_index,
                action_index,
            } => [8, offset as u32, rule_index as u32, action_index as u32],
        }
    }
}
