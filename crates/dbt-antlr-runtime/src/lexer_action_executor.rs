use std::hash::{Hash, Hasher};

use fxhash::{hash64, FxHasher64};

use crate::char_stream::CharStream;
use crate::dfa::FinalizeStore;
use crate::lexer::Lexer;
use crate::lexer_action::LexerAction;
use crate::token_factory::TokenFactory;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LexerActionExecutor<'ephemeral> {
    cached_hash: u64,
    lexer_actions: &'ephemeral [LexerAction],
}

impl Hash for LexerActionExecutor<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.cached_hash)
    }
}

impl<'ephemeral> LexerActionExecutor<'ephemeral> {
    pub(crate) fn new_copy_append(
        scratch: &'ephemeral bumpalo::Bump,
        old: Option<&Self>,
        lexer_action: LexerAction,
    ) -> LexerActionExecutor<'ephemeral> {
        if let Some(LexerActionExecutor {
            lexer_actions,
            cached_hash,
        }) = old
        {
            let new_hash = {
                let mut hasher = FxHasher64::default();
                hasher.write_u64(*cached_hash);
                lexer_action.hash(&mut hasher);
                hasher.finish()
            };
            let new_actions = scratch.alloc_slice_fill_with(lexer_actions.len() + 1, |i| {
                if i < lexer_actions.len() {
                    lexer_actions[i]
                } else {
                    lexer_action
                }
            });

            LexerActionExecutor {
                lexer_actions: new_actions,
                cached_hash: new_hash,
            }
        } else {
            LexerActionExecutor {
                cached_hash: { hash64(&lexer_action) },
                lexer_actions: scratch.alloc_slice_fill_iter(std::iter::once(lexer_action)),
            }
        }
    }

    #[cfg(feature = "static-dfa-build")]
    pub(crate) fn actions(&self) -> &'ephemeral [LexerAction] {
        self.lexer_actions
    }

    pub fn fix_offset_before_match(
        &self,
        arena: &'ephemeral bumpalo::Bump,
        offset: isize,
    ) -> LexerActionExecutor<'ephemeral> {
        let offset = i32::try_from(offset).expect("lexer action offset fits in i32");
        let fixed_actions = arena.alloc_slice_fill_with(self.lexer_actions.len(), |i| {
            match self.lexer_actions[i] {
                LexerAction::Custom {
                    rule_index,
                    action_index,
                } => LexerAction::IndexedCustom {
                    offset,
                    rule_index,
                    action_index,
                },
                action => action,
            }
        });
        LexerActionExecutor {
            lexer_actions: fixed_actions,
            cached_hash: self.cached_hash,
        }
    }

    pub fn execute<'input, 'arena, Input, TF>(
        &self,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        start_index: isize,
    ) where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let mut requires_seek = false;
        let stop_index = lexer.input().index();
        for action in self.lexer_actions.iter() {
            if let LexerAction::IndexedCustom { offset, .. } = *action {
                let offset = offset as isize;
                lexer.input().seek(start_index + offset);
                requires_seek = start_index + offset != stop_index;
            } else if action.is_position_dependent() {
                lexer.input().seek(stop_index);
                requires_seek = false
            }
            action.execute(lexer);
        }
        if requires_seek {
            lexer.input().seek(stop_index);
        }
    }

    pub(crate) fn promote<'sim>(
        &self,
        store: &impl FinalizeStore<'sim>,
    ) -> LexerActionExecutor<'sim> {
        if store.contains_lexer_action_slice(self.lexer_actions) {
            // Safety: the actions are already in the target arena, so can
            // live as long as the target lifetime:
            return unsafe { std::mem::transmute::<Self, LexerActionExecutor<'sim>>(self.clone()) };
        }

        let promoted_actions =
            store.alloc_lexer_action_slice(self.lexer_actions.len(), |i| self.lexer_actions[i]);
        LexerActionExecutor {
            lexer_actions: promoted_actions,
            cached_hash: self.cached_hash,
        }
    }
}
