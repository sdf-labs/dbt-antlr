//! Static analysis seam for the tool-side parser DFA classifier.
//! (Feature `static-dfa-build`.)
//!
//! `dbt-antlr-codegen` classifies the prediction decisions of a compiled parser
//! at build time. The analysis is *not* a second DFA implementation: the
//! classifier drives the runtime's own machinery.
//!
//! 1. A [`ParserATNSimulatorManager`] is created from the compiled unit's
//!    `&'static ATN`; its `get_simulator` borrows an [`Arena`] and hands out the
//!    parser [`ParserATNSimulator`].
//! 2. The classifier interns its DFA states and token edges through [`DFA`]
//!    itself (`DFA::add_state`, `DFA::set_edge`), so state deduplication, the
//!    edge store, and the state budget accounting are the runtime's.
//! 3. Closures run on [`MutableATNConfigSet`] with the same
//!    [`MergeCache`] context merging as the lazy simulator, and conflicts are
//!    decided by the same `prediction_mode` predicates the runtime uses.
//!
//! What the classifier adds on top is the part the lazy simulator cannot
//! express, because it has no input: the subset construction over *token
//! intervals*, the depth/size budgets that bound it on recursive grammars,
//! and the taint bits (see [`BOUNDARY_TAINT`]) that record where the
//! construction had to approximate. Everything in this module is
//! crate-private machinery re-exported for the classifier; nothing here is
//! meant for generated recognizers.

pub use bit_set::BitSet;
pub use bumpalo::Bump;

pub use crate::arena::Arena;
pub use crate::atn::{ATN, INVALID_ALT};
pub use crate::atn_config::{
    ATNConfig, BOUNDARY_TAINT, FOREIGN_CONSUME_TAINT, INTERIOR_TAINT, PRECPRED_TAINT, PRED_TAINT,
    ROOTED_TAINT, WIDENED_TAINT,
};
pub use crate::atn_config_set::{ATNConfigSet, MutableATNConfigSet};
pub use crate::atn_simulator::IATNSimulator;
pub use crate::atn_simulator::ParserATNSimulatorManager;
pub use crate::atn_state::{
    ATNStateRef, ATNStateType, BasicBlockStartState, PlusBlockStartState, RuleStartState,
    StarBlockStartState, StarLoopEntryState,
};
pub use crate::dfa::{DFAState, ProposedDFAState, DFA};
pub use crate::int_stream::EOF;
pub use crate::interval_set::{IntervalSet, IntervalSetBuf};
pub use crate::parser_atn_simulator::{MergeCache, ParserATNSimulator};
pub use crate::prediction_context::{PredictionContext, PredictionContextRef};
pub use crate::prediction_mode::{
    all_subsets_conflict, all_subsets_equal, get_alts, has_sll_conflict_terminating_prediction,
    PredictionMode,
};

/// The conflicting alternative subsets of an SLL conflict: one set of
/// alternatives per `(state, context)` key of the config set.
pub fn conflicting_alt_subsets<'a>(
    scratch: &'a Bump,
    configs: &MutableATNConfigSet<'a>,
) -> Vec<BitSet> {
    crate::prediction_mode::get_conflicting_alt_subsets(scratch, configs)
        .into_values()
        .collect()
}
pub use crate::token::TOKEN_MIN_USER_TOKEN_TYPE;
pub use crate::transition::{
    NotSetTransition, PrecedencePredicateTransition, PredicateTransition, RuleTransition,
    Transition, TransitionType,
};
