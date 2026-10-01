//! Normalized plain-text dump of an [`ATN`] graph.
//!
//! Differential tests compare the dumps of the two ATN deserializers. The
//! format is deterministic: one line per item, `\n`-separated, with no
//! trailing spaces. Derived and lazy fields (`epsilon_only_transitions`,
//! `next_tokens_within_rule`, `mode_name_to_start_state`) are not part of
//! the dump.

use std::fmt::Write;

use crate::atn::ATN;
use crate::atn_state::ATNState;
use crate::atn_state::ATNStateRef;
use crate::atn_state::ATNStateType;
use crate::atn_state::BlockEndState;
use crate::atn_state::LoopEndState;
use crate::atn_state::PlusBlockStartState;
use crate::atn_state::RuleStartState;
use crate::atn_state::StarLoopEntryState;
use crate::atn_type::ATNType;
use crate::lexer_action::LexerAction;
use crate::transition::ActionTransition;
use crate::transition::AtomTransition;
use crate::transition::PrecedencePredicateTransition;
use crate::transition::PredicateTransition;
use crate::transition::RangeTransition;
use crate::transition::RuleTransition;
use crate::transition::Transition;
use crate::transition::TransitionType;

/// Dumps `atn` in the normalized text format.
///
/// The output covers the grammar type, the rule, mode, and decision tables,
/// every state in state-number order with its outgoing transitions, and the
/// lexer actions of lexer ATNs.
pub fn dump_atn(atn: &ATN) -> String {
    let mut out = String::new();
    let grammar_type = match atn.grammar_type {
        ATNType::Lexer => "LEXER",
        ATNType::Parser => "PARSER",
    };
    writeln!(
        out,
        "atn grammar_type={} max_token_type={}",
        grammar_type, atn.max_token_type
    )
    .unwrap();
    for (i, start) in atn.rule_to_start_state.iter().enumerate() {
        write!(out, "rule {}: start={}", i, start.get_state_number()).unwrap();
        if let Some(stop) = atn.rule_to_stop_state.get(i) {
            write!(out, " stop={}", stop.get_state_number()).unwrap();
        }
        if let Some(token_type) = atn.rule_to_token_type.get(i) {
            write!(out, " token_type={}", token_type).unwrap();
        }
        writeln!(out).unwrap();
    }
    for (i, start) in atn.mode_to_start_state.iter().enumerate() {
        writeln!(out, "mode {}: start={}", i, start.get_state_number()).unwrap();
    }
    for (i, state) in atn.decision_to_state.iter().enumerate() {
        writeln!(out, "decision {}: state={}", i, state.get_state_number()).unwrap();
    }
    for state in atn.iter_states() {
        dump_state(&mut out, state);
    }
    for (i, action) in atn.lexer_actions.iter().enumerate() {
        let (kind, data1, data2) = match action {
            LexerAction::Channel(d) => ("CHANNEL", *d, -1),
            LexerAction::Custom {
                rule_index,
                action_index,
            } => ("CUSTOM", *rule_index, *action_index),
            LexerAction::Mode(d) => ("MODE", *d, -1),
            LexerAction::More => ("MORE", -1, -1),
            LexerAction::PopMode => ("POP_MODE", -1, -1),
            LexerAction::PushMode(d) => ("PUSH_MODE", *d, -1),
            LexerAction::Skip => ("SKIP", -1, -1),
            LexerAction::Type(d) => ("TYPE", *d, -1),
            LexerAction::IndexedCustom(a) => ("INDEXED_CUSTOM", a.offset as i32, -1),
        };
        writeln!(
            out,
            "lexer_action {}: {} data1={} data2={}",
            i, kind, data1, data2
        )
        .unwrap();
    }
    out
}

fn dump_state(out: &mut String, state: &ATNState) {
    write!(
        out,
        "state {}: {} rule={}",
        state.get_state_number(),
        state_type_name(state.state_type()),
        state.get_rule_index()
    )
    .unwrap();
    match state.state_type() {
        ATNStateType::RuleStart => {
            let s = state.try_as::<RuleStartState>().unwrap();
            out.push_str(" stop=");
            push_link(out, s.stop_state);
            write!(out, " left_recursive={}", s.is_left_recursive).unwrap();
        }
        ATNStateType::BasicBlockStart
        | ATNStateType::StarBlockStart
        | ATNStateType::PlusBlockStart => {
            out.push_str(" decision=");
            write!(
                out,
                "{} nongreedy={} end=",
                state.get_decision().unwrap(),
                state.is_nongreedy_decision().unwrap()
            )
            .unwrap();
            push_link(out, state.get_decision_end_state().unwrap());
            if let Some(s) = state.try_as::<PlusBlockStartState>() {
                out.push_str(" loop_back=");
                push_link(out, s.loop_back_state);
            }
        }
        ATNStateType::StarLoopEntry => {
            let s = state.try_as::<StarLoopEntryState>().unwrap();
            out.push_str(" decision=");
            write!(
                out,
                "{} nongreedy={} precedence={} loop_back=",
                s.decision, s.nongreedy, s.is_precedence
            )
            .unwrap();
            push_link(out, s.loop_back_state);
        }
        ATNStateType::TokenStart | ATNStateType::PlusLoopBack => {
            out.push_str(" decision=");
            write!(
                out,
                "{} nongreedy={}",
                state.get_decision().unwrap(),
                state.is_nongreedy_decision().unwrap()
            )
            .unwrap();
        }
        ATNStateType::LoopEnd => {
            let s = state.try_as::<LoopEndState>().unwrap();
            out.push_str(" loop_back=");
            push_link(out, s.loop_back_state);
        }
        ATNStateType::BlockEnd => {
            let s = state.try_as::<BlockEndState>().unwrap();
            out.push_str(" end=");
            push_link(out, s.end_state);
        }
        _ => (),
    }
    writeln!(out).unwrap();

    let mut transitions: Vec<&Transition> = state.get_transitions().iter().collect();
    if state.state_type() == ATNStateType::RuleStop {
        // Rule-return edges are synthesized by the deserializers. Their order
        // is an artifact of construction order (Java-word edge order vs packed
        // sorted-by-source order) and carries no semantics, so normalize it.
        transitions.sort_by_key(|tr| tr.get_target().get_state_number());
    }
    for tr in transitions {
        dump_transition(out, tr);
    }
}

fn dump_transition(out: &mut String, tr: &Transition) {
    write!(
        out,
        "  -> {} target={}",
        transition_type_name(tr.transition_type()),
        tr.get_target().get_state_number()
    )
    .unwrap();
    match tr.transition_type() {
        TransitionType::Atom => {
            let t = tr.try_as::<AtomTransition>().unwrap();
            write!(out, " label={}", t.label()).unwrap();
        }
        TransitionType::Range => {
            let t = tr.try_as::<RangeTransition>().unwrap();
            write!(out, " start={} stop={}", t.start(), t.stop()).unwrap();
        }
        TransitionType::Rule => {
            let t = tr.try_as::<RuleTransition>().unwrap();
            write!(
                out,
                " rule={} follow={} precedence={}",
                t.rule_index(),
                t.follow_state.get_state_number(),
                t.precedence()
            )
            .unwrap();
        }
        TransitionType::Predicate => {
            let t = tr.try_as::<PredicateTransition>().unwrap();
            write!(
                out,
                " rule={} pred={} ctx={}",
                t.rule_index(),
                t.pred_index(),
                t.is_ctx_dependent()
            )
            .unwrap();
        }
        TransitionType::Action => {
            let t = tr.try_as::<ActionTransition>().unwrap();
            write!(
                out,
                " rule={} action={} ctx={}",
                t.rule_index(),
                t.action_index(),
                t.is_ctx_dependent
            )
            .unwrap();
        }
        TransitionType::Set | TransitionType::NotSet => {
            out.push_str(" set=[");
            for (i, interval) in tr.get_label().unwrap().as_slice().iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write!(out, "({},{})", interval.a, interval.b).unwrap();
            }
            out.push(']');
        }
        TransitionType::PrecedencePredicate => {
            let t = tr.try_as::<PrecedencePredicateTransition>().unwrap();
            write!(out, " precedence={}", t.precedence()).unwrap();
        }
        TransitionType::Epsilon | TransitionType::Wildcard => (),
    }
    writeln!(out).unwrap();
}

fn push_link(out: &mut String, state: ATNStateRef) {
    if state == ATNStateRef::invalid() {
        out.push('-');
    } else {
        write!(out, "{}", state.get_state_number()).unwrap();
    }
}

fn state_type_name(state_type: ATNStateType) -> &'static str {
    match state_type {
        ATNStateType::RuleStart => "RuleStart",
        ATNStateType::RuleStop => "RuleStop",
        ATNStateType::BlockEnd => "BlockEnd",
        ATNStateType::LoopEnd => "LoopEnd",
        ATNStateType::StarLoopback => "StarLoopback",
        ATNStateType::Basic => "Basic",
        ATNStateType::StarLoopEntry => "StarLoopEntry",
        ATNStateType::TokenStart => "TokenStart",
        ATNStateType::PlusLoopBack => "PlusLoopBack",
        ATNStateType::BasicBlockStart => "BasicBlockStart",
        ATNStateType::StarBlockStart => "StarBlockStart",
        ATNStateType::PlusBlockStart => "PlusBlockStart",
        ATNStateType::Invalid => "Invalid",
    }
}

fn transition_type_name(transition_type: TransitionType) -> &'static str {
    match transition_type {
        TransitionType::Atom => "ATOM",
        TransitionType::Rule => "RULE",
        TransitionType::Epsilon => "EPSILON",
        TransitionType::Range => "RANGE",
        TransitionType::Action => "ACTION",
        TransitionType::Set => "SET",
        TransitionType::NotSet => "NOT_SET",
        TransitionType::Wildcard => "WILDCARD",
        TransitionType::Predicate => "PREDICATE",
        TransitionType::PrecedencePredicate => "PRECEDENCE",
    }
}
