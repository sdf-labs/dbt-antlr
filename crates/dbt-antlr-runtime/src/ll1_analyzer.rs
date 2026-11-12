use std::rc::Rc;

use bit_set::BitSet;
use hashbrown::DefaultHashBuilder;
use hashbrown::HashMap;
use hashbrown::HashSet;

use crate::atn::ATN;
use crate::atn_config::ATNConfig;
use crate::atn_state::ATNStateRef;
use crate::atn_state::ATNStateType;
use crate::interval_set::IntervalSetBuf;
use crate::prediction_context::PredictionContext;
use crate::prediction_context::PredictionContextRef;
use crate::token::Token;
use crate::token::{TOKEN_EOF, TOKEN_EPSILON, TOKEN_INVALID_TYPE, TOKEN_MIN_USER_TOKEN_TYPE};
use crate::transition::TransitionType;
use crate::tree::NodeKindType;
use crate::tree::TreeNode;

pub struct LL1Analyzer<'a> {
    atn: &'a ATN,
}

impl LL1Analyzer<'_> {
    pub fn new(atn: &ATN) -> LL1Analyzer<'_> {
        LL1Analyzer { atn }
    }

    //    fn get_decision_lookahead(&self, _s: &dyn ATNState) -> &Vec<IntervalSet> { unimplemented!() }

    pub fn look<'input, 'arena, Node, Tok>(
        &self,
        s: ATNStateRef,
        stop_state: Option<ATNStateRef>,
        ctx: Option<&'arena TreeNode<'input, 'arena, Node, Tok>>,
    ) -> IntervalSetBuf
    where
        'input: 'arena,
        Node: NodeKindType<'arena, Tok>,
        Tok: Token + 'input,
    {
        let arena = bumpalo::Bump::new();

        let mut r = IntervalSetBuf::new();
        let look_ctx = ctx.map(|x| PredictionContext::from_rule_context(self.atn, x, &arena));
        let mut looks_busy: HashSet<ATNConfig<'_>, _, &bumpalo::Bump> = HashSet::new_in(&arena);
        let mut rule_looks = HashMap::new();

        self.look_work(
            s,
            stop_state,
            look_ctx,
            &mut r,
            &mut looks_busy,
            &mut rule_looks,
            true,
            true,
        );
        r
    }

    #[allow(clippy::too_many_arguments)]
    fn look_work<'ephemeral>(
        &self,
        s: ATNStateRef,
        stop_state: Option<ATNStateRef>,
        ctx: Option<PredictionContextRef<'ephemeral>>,
        look: &mut IntervalSetBuf,
        look_busy: &mut HashSet<ATNConfig<'ephemeral>, DefaultHashBuilder, &bumpalo::Bump>,
        rule_looks: &mut RuleLooks,
        see_thru_preds: bool,
        add_eof: bool,
    ) {
        let c = ATNConfig::new(s, 0, ctx);
        if !look_busy.insert(c) {
            return;
        }

        if Some(s.get_state_number()) == stop_state.map(|x| x.get_state_number()) {
            match ctx {
                None => {
                    look.add_one(TOKEN_EPSILON);
                    return;
                }
                Some(x) if x.is_empty() && add_eof => {
                    look.add_one(TOKEN_EOF);
                    return;
                }
                _ => {}
            }
        }

        if s.state_type() == ATNStateType::RuleStop {
            match ctx {
                None => {
                    look.add_one(TOKEN_EPSILON);
                    return;
                }
                Some(x) if x.is_empty() && add_eof => {
                    look.add_one(TOKEN_EOF);
                    return;
                }
                Some(ctx) if ctx != PredictionContextRef::new_empty() => {
                    for i in 0..ctx.length() {
                        self.look_work(
                            ctx.get_return_state(i),
                            stop_state,
                            ctx.get_parent(i),
                            look,
                            look_busy,
                            rule_looks,
                            see_thru_preds,
                            add_eof,
                        )
                    }
                    return;
                }
                _ => {}
            }
        }

        for tr in s.get_transitions() {
            let target = tr.get_target();
            match tr.transition_type() {
                TransitionType::Rule => {
                    let rule_tr = tr.try_as::<crate::transition::RuleTransition>().unwrap();

                    let rule = target.get_rule_index() as usize;
                    debug_assert_eq!(target, self.atn.rule_to_start_state[rule]);
                    let callee =
                        self.rule_look(rule, &mut BitSet::new(), rule_looks, see_thru_preds);
                    look.add_set(&callee.tokens);
                    if callee.reaches_stop {
                        self.look_work(
                            rule_tr.follow_state,
                            stop_state,
                            ctx,
                            look,
                            look_busy,
                            rule_looks,
                            see_thru_preds,
                            add_eof,
                        );
                    }
                }
                TransitionType::Predicate | TransitionType::PrecedencePredicate => {
                    if see_thru_preds {
                        self.look_work(
                            target,
                            stop_state,
                            ctx,
                            look,
                            look_busy,
                            rule_looks,
                            see_thru_preds,
                            add_eof,
                        )
                    } else {
                        look.add_one(TOKEN_INVALID_TYPE);
                    }
                }
                TransitionType::Wildcard => {
                    look.add_range(TOKEN_MIN_USER_TOKEN_TYPE, self.atn.max_token_type)
                }
                _ if tr.is_epsilon() => self.look_work(
                    target,
                    stop_state,
                    ctx,
                    look,
                    look_busy,
                    rule_looks,
                    see_thru_preds,
                    add_eof,
                ),
                TransitionType::NotSet => {
                    let set = tr.get_label().unwrap();
                    look.add_set(
                        &set.complement(TOKEN_MIN_USER_TOKEN_TYPE, self.atn.max_token_type),
                    )
                }
                _ => {
                    if let Some(set) = tr.get_label() {
                        look.add_set(set)
                    }
                }
            }
        }
    }

    /// Returns the look of `rule` from its start state to its stop state.
    ///
    /// `called_rule_stack` marks the rules on the call stack. The look does
    /// not enter a marked rule again.
    fn rule_look(
        &self,
        rule: usize,
        called_rule_stack: &mut BitSet,
        rule_looks: &mut RuleLooks,
        see_thru_preds: bool,
    ) -> Rc<RuleLook> {
        called_rule_stack.insert(rule);
        if let Some(known) = rule_looks.get(&rule).and_then(|looks| {
            looks
                .iter()
                .find(|look| look.matches(see_thru_preds, called_rule_stack))
        }) {
            let known = Rc::clone(known);
            called_rule_stack.remove(rule);
            return known;
        }

        let mut tokens = IntervalSetBuf::new();
        let mut tested = BitSet::new();
        let mut reaches_stop = false;
        let mut pending = vec![self.atn.rule_to_start_state[rule]];
        let mut visited = HashSet::new();
        while let Some(s) = pending.pop() {
            if !visited.insert(s.get_state_number()) {
                continue;
            }
            if s.state_type() == ATNStateType::RuleStop {
                reaches_stop = true;
                continue;
            }
            for tr in s.get_transitions() {
                let target = tr.get_target();
                match tr.transition_type() {
                    TransitionType::Rule => {
                        let rule_tr = tr.try_as::<crate::transition::RuleTransition>().unwrap();
                        let callee_rule = target.get_rule_index() as usize;
                        tested.insert(callee_rule);
                        if called_rule_stack.contains(callee_rule) {
                            continue;
                        }
                        debug_assert_eq!(target, self.atn.rule_to_start_state[callee_rule]);
                        let callee = self.rule_look(
                            callee_rule,
                            called_rule_stack,
                            rule_looks,
                            see_thru_preds,
                        );
                        tested.extend(callee.tested.iter().map(|(rule, _)| *rule));
                        tokens.add_set(&callee.tokens);
                        if callee.reaches_stop {
                            pending.push(rule_tr.follow_state);
                        }
                    }
                    TransitionType::Predicate | TransitionType::PrecedencePredicate => {
                        if see_thru_preds {
                            pending.push(target);
                        } else {
                            tokens.add_one(TOKEN_INVALID_TYPE);
                        }
                    }
                    TransitionType::Wildcard => {
                        tokens.add_range(TOKEN_MIN_USER_TOKEN_TYPE, self.atn.max_token_type)
                    }
                    _ if tr.is_epsilon() => pending.push(target),
                    TransitionType::NotSet => {
                        let set = tr.get_label().unwrap();
                        tokens.add_set(
                            &set.complement(TOKEN_MIN_USER_TOKEN_TYPE, self.atn.max_token_type),
                        )
                    }
                    _ => {
                        if let Some(set) = tr.get_label() {
                            tokens.add_set(set)
                        }
                    }
                }
            }
        }

        let look = Rc::new(RuleLook {
            see_thru_preds,
            tested: tested
                .iter()
                .map(|tested| (tested, called_rule_stack.contains(tested)))
                .collect(),
            tokens,
            reaches_stop,
        });
        called_rule_stack.remove(rule);
        rule_looks.entry(rule).or_default().push(Rc::clone(&look));
        look
    }
}

type RuleLooks = HashMap<usize, Vec<Rc<RuleLook>>>;

/// The look of one called rule, from its start state to its stop state.
///
/// A look inside a called rule does not read the frames of its callers. The
/// result depends only on the rule, on the predicate mode, and on the marked
/// rules that the look does not enter. `tested` holds each rule that the look
/// tried to call, and whether that rule was marked. The result is correct for
/// each call stack that agrees on all of these rules.
struct RuleLook {
    see_thru_preds: bool,
    tested: Vec<(usize, bool)>,
    tokens: IntervalSetBuf,
    reaches_stop: bool,
}

impl RuleLook {
    fn matches(&self, see_thru_preds: bool, called_rule_stack: &BitSet) -> bool {
        self.see_thru_preds == see_thru_preds
            && self
                .tested
                .iter()
                .all(|(rule, was_marked)| called_rule_stack.contains(*rule) == *was_marked)
    }
}
