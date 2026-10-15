use std::hash::Hash;

use crate::dfa::FinalizeStore;
use crate::lexer::Lexer;

pub(crate) const LEXER_ACTION_TYPE_CHANNEL: i32 = 0;
pub(crate) const LEXER_ACTION_TYPE_CUSTOM: i32 = 1;
pub(crate) const LEXER_ACTION_TYPE_MODE: i32 = 2;
pub(crate) const LEXER_ACTION_TYPE_MORE: i32 = 3;
pub(crate) const LEXER_ACTION_TYPE_POP_MODE: i32 = 4;
pub(crate) const LEXER_ACTION_TYPE_PUSH_MODE: i32 = 5;
pub(crate) const LEXER_ACTION_TYPE_SKIP: i32 = 6;
pub(crate) const LEXER_ACTION_TYPE_TYPE: i32 = 7;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Default, Eq, Debug, Hash)]
pub(crate) enum LexerAction<'ephemeral> {
    Channel(i32),
    Custom {
        rule_index: i32,
        action_index: i32,
    },
    Mode(i32),
    More,
    PopMode,
    PushMode(i32),
    #[default]
    Skip,
    Type(i32),
    IndexedCustom(&'ephemeral LexerIndexedCustomAction<'ephemeral>),
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub(crate) struct LexerIndexedCustomAction<'ephemeral> {
    pub(crate) offset: isize,
    pub(crate) action: LexerAction<'ephemeral>,
}

// Lifetime-agnostic equality: `IndexedCustom` compares by pointed-to value
// (offset + action recursively), matching the derived `Hash`, which hashes
// through the reference.
impl<'a, 'b> PartialEq<LexerAction<'b>> for LexerAction<'a> {
    fn eq(&self, other: &LexerAction<'b>) -> bool {
        match (self, other) {
            (&LexerAction::Channel(a), &LexerAction::Channel(b)) => a == b,
            (
                &LexerAction::Custom {
                    rule_index: a_rule,
                    action_index: a_action,
                },
                &LexerAction::Custom {
                    rule_index: b_rule,
                    action_index: b_action,
                },
            ) => a_rule == b_rule && a_action == b_action,
            (&LexerAction::Mode(a), &LexerAction::Mode(b)) => a == b,
            (LexerAction::More, LexerAction::More) => true,
            (LexerAction::PopMode, LexerAction::PopMode) => true,
            (&LexerAction::PushMode(a), &LexerAction::PushMode(b)) => a == b,
            (LexerAction::Skip, LexerAction::Skip) => true,
            (&LexerAction::Type(a), &LexerAction::Type(b)) => a == b,
            (LexerAction::IndexedCustom(a), LexerAction::IndexedCustom(b)) => {
                a.offset == b.offset && a.action == b.action
            }
            _ => false,
        }
    }
}

impl<'ephemeral> LexerAction<'ephemeral> {
    /// The ATN `(type, data1, data2)` encoding of this action, the inverse
    /// of [`Self::from_serialized`].
    ///
    /// Panics on `IndexedCustom`: a position-fixed action has no ATN
    /// encoding (serialize the underlying custom action instead).
    #[cfg(feature = "static-dfa-build")]
    pub(crate) fn to_serialized(&self) -> (i32, i32, i32) {
        match self {
            &LexerAction::Channel(channel) => (LEXER_ACTION_TYPE_CHANNEL, channel, 0),
            &LexerAction::Custom {
                rule_index,
                action_index,
            } => (LEXER_ACTION_TYPE_CUSTOM, rule_index, action_index),
            &LexerAction::Mode(mode) => (LEXER_ACTION_TYPE_MODE, mode, 0),
            LexerAction::More => (LEXER_ACTION_TYPE_MORE, 0, 0),
            LexerAction::PopMode => (LEXER_ACTION_TYPE_POP_MODE, 0, 0),
            &LexerAction::PushMode(mode) => (LEXER_ACTION_TYPE_PUSH_MODE, mode, 0),
            LexerAction::Skip => (LEXER_ACTION_TYPE_SKIP, 0, 0),
            &LexerAction::Type(ttype) => (LEXER_ACTION_TYPE_TYPE, ttype, 0),
            LexerAction::IndexedCustom(_) => {
                unreachable!("position-fixed actions have no ATN encoding")
            }
        }
    }

    /// Rebuilds one action from its ATN `(type, data1, data2)` encoding.
    ///
    /// Panics on an unknown action type: the encoding is produced together
    /// with the ATN, so an invalid value is a build inconsistency, not a
    /// runtime condition.
    pub(crate) fn from_serialized(
        action_type: i32,
        data1: i32,
        data2: i32,
    ) -> LexerAction<'static> {
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

    //    fn get_action_type(&self) -> i32 {
    //        unimplemented!()
    ////        unsafe {discriminant_value(self)} as i32
    //    }
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
        match self {
            &LexerAction::Channel(channel) => lexer.set_channel(channel),
            &LexerAction::Custom {
                rule_index,
                action_index,
            } => {
                lexer.action(None, rule_index, action_index);
            }
            &LexerAction::Mode(mode) => lexer.set_mode(mode as usize),
            &LexerAction::More => lexer.more(),
            &LexerAction::PopMode => {
                lexer.pop_mode();
            }
            &LexerAction::PushMode(mode) => lexer.push_mode(mode as usize),
            &LexerAction::Skip => lexer.skip(),
            &LexerAction::Type(ty) => lexer.set_type(ty),
            LexerAction::IndexedCustom(action) => action.action.execute(lexer),
        }
    }

    pub(crate) fn promote<'sim>(&self, store: &impl FinalizeStore<'sim>) -> LexerAction<'sim> {
        match self {
            LexerAction::IndexedCustom(action)
                if store.contains_lexer_indexed_custom_action(action) =>
            {
                // Safety: the action is already in the target arena, so can
                // live as long as the target lifetime:
                unsafe { std::mem::transmute::<Self, LexerAction<'sim>>(self.clone()) }
            }
            LexerAction::IndexedCustom(action) => {
                LexerAction::IndexedCustom(store.alloc_lexer_indexed_custom_action(|| {
                    LexerIndexedCustomAction {
                        offset: action.offset,
                        action: action.action.promote(store),
                    }
                }))
            }
            // Safety: these don't hold any references, so effectively 'static
            _ => unsafe { std::mem::transmute::<Self, LexerAction<'sim>>(self.clone()) },
        }
    }
}
