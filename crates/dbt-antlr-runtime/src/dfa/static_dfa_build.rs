//! Compile-time static lexer DFA builder (feature `static-dfa-build`).
//!
//! The builder takes a lexer [`ATN`] and runs the subset construction that the
//! lazy [`crate::lexer_atn_simulator::LexerATNSimulator`] runs at runtime. It
//! drives the move and closure code of the simulator itself, so maximal munch,
//! rule priority, non-greedy loops, and action accumulation match the lazy
//! path by construction. The result is one [`StaticLexerDFA`] table per lexer
//! mode plus the `Vec<u32>` word stream that generated code embeds
//! ([`StaticLexerTables::to_words`]). The walker's `match_static` tokenizes
//! from the tables without ATN simulation.
//!
//! # Build structure
//!
//! The build carries structural data only. [`ModeBuild`] holds finalized
//! [`LexerATNConfig`] sets, structured edges ([`EdgeEntry`]), accept data as
//! ATN action indices, and continuations as [`PendingContinuation`]. State
//! and continuation interning dedupe by structural content: every config set
//! is finalized first through a [`PredictionContextCache`] and a build-side
//! [`BuildStore`] (one per mode; its arena backs both the scratch and the
//! finalized configs), so contexts are hash-consed and executors and
//! semantic contexts interned.
//! [`ModeBuild::finalize`] then lays out the walker's own [`StaticLexerDFA`]:
//! `u16` packing with the `NO_EDGE`/`ESCAPE`/`ACCEPT_BIT` sentinels, the
//! dense 129-wide low rows, the hi-class compression, the deduplicated
//! accept action tables, the accel classification, the escape ranges, and
//! the continuation pool. The word-stream encoding of the tables lives with
//! the decoder: `StaticLexerTables::to_words` in [`crate::dfa::static_lexer_dfa`],
//! whose module documentation is the authoritative wire-format contract.
//!
//! # Escape edges
//!
//! Some edges cannot get a static target. Their table entry is the `ESCAPE`
//! sentinel, and the walker hands the token back to the lazy simulator at the
//! escape point. The escape row of the state names a continuation: the
//! serialized post-move, pre-closure configs of the edge. The walker rebuilds
//! the configs, runs the real closure, and continues lazily. Four triggers
//! escape an edge:
//!
//! 1. The closure crossed a predicate. Build-time closures evaluate predicates
//!    as `true` and flag the edge. The edge escapes because the runtime result
//!    can differ.
//! 2. A closed config has a recursive context: a repeated return state on the
//!    context path, or a context deeper than 32 frames.
//! 3. A position-dependent (custom) lexer action appears in the action
//!    accumulation of a closed config. A later phase can replace this rule
//!    with the `behind` form of the action trace: serialize the action with a
//!    relative offset and keep the edge static.
//! 4. The per-mode state budget is spent. Budget escapes carry no
//!    continuation: the walker restarts the token lazily.
//!
//! When the move step of an escape edge pruned a config (the `skip_alt` rule
//! fired, or the closure-driven early break fired), the moved configs bake in
//! a build-time pruning decision. The continuation then holds the source
//! config set of the state instead, and the resume re-runs the full move step
//! (`CONTINUATION_SOURCE_SET`). Eligibility is per edge. A whole mode fails
//! only when the start-state closure itself escapes.
//!
//! # EOF edges
//!
//! EOF consumes no input. The builder computes EOF edges with the
//! `treat_eofas_epsilon` semantics of the lazy simulator. The walker must not
//! advance the cursor on an EOF edge, must stop the walk on an EOF accept, and
//! must bound the count of consecutive EOF edges it takes (grammars can have
//! unbounded EOF epsilon chains). Past the bound, the walker restarts the
//! token lazily. The table itself needs no extra data for this rule.
//!
//! The build is deterministic: identical input ATNs produce identical words.

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};

use crate::arena::{is_ref_in_arena, is_slice_in_arena};
use crate::atn::ATN;
use crate::atn_config::LexerATNConfig;
use crate::atn_config_set::MutableLexerATNConfigSet;
use crate::atn_state::{ATNStateRef, ATNStateType, StarLoopEntryState};
use crate::dfa::static_lexer_dfa::{
    class_map, dense_hi_rows, Accel, ContinuationConfig, EscapeRange, LexerContinuation,
    StaticLexerDFA, StaticLexerTables, ACCEPT_BIT, CONTINUATION_MOVED_CONFIGS,
    CONTINUATION_SOURCE_SET, ESCAPE, LOW_ROW, NO_CONTINUATION, NO_EDGE,
};
use crate::dfa::FinalizeStore;
use crate::int_stream::EOF;
use crate::lexer_action::{LexerAction, LexerIndexedCustomAction};
use crate::lexer_action_executor::LexerActionExecutor;
use crate::lexer_atn_simulator::{reachable_core, start_state_core, LexerClosureEnv, ReachCapture};
use crate::prediction_context::PredictionContextRef;
use crate::semantic_context::SemanticContext;
use crate::transition::{
    ActionTransition, AtomTransition, RuleTransition, Transition, TransitionType,
};
use crate::PredictionContextCache;

/// The default per-mode DFA state budget
pub const DEFAULT_MAX_STATES_PER_MODE: usize = 4096;

const MAX_SYMBOL: i32 = 0x10FFFF;
const ASCII_MAX: i32 = 127;
const MAX_TABLE_STATE_COUNT: usize = 0x7FFE;
const MAX_CONTEXT_DEPTH: u32 = 32;

/// The result of [`StaticLexerDfaBuilder::build`]: the word-stream encoding
/// of the committed tables, and the per-mode build outcomes (each carrying
/// its committed table).
#[derive(Debug)]
pub struct StaticLexerDfaBuild {
    /// The word-stream encoding of the tables of `modes`
    /// (`StaticLexerTables::to_words`), in the format of the
    /// [`crate::dfa::static_lexer_dfa`] module documentation.
    pub words: Vec<u32>,
    /// Per-mode build outcomes, in mode order.
    pub modes: Vec<ModeBuildOutcome>,
}

impl StaticLexerDfaBuild {
    /// The committed table of one mode, when the mode has one.
    pub fn table(&self, mode: usize) -> Option<&StaticLexerDFA> {
        self.modes.get(mode)?.table.as_ref()
    }
}

/// The build outcome of one mode.
#[derive(Debug)]
pub struct ModeBuildOutcome {
    /// The committed table of the mode; `None` when the mode has no table.
    pub table: Option<StaticLexerDFA>,
    /// Why the mode has no table.
    pub failure: Option<ModeFailure>,
    /// The number of states of the table.
    pub num_states: usize,
    /// The number of continuations of the mode.
    pub num_continuations: usize,
    /// Escape edge counts by reason.
    pub escape_counts: EscapeCounts,
}

/// Why a whole mode has no table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeFailure {
    /// The start-state closure crossed a predicate.
    StartClosurePredicate,
    /// The start-state closure accumulated a position-dependent action.
    StartClosureCustomAction,
    /// The start-state closure produced a recursive context.
    StartClosureRecursiveContext,
    /// The state budget did not cover the start state.
    StartStateBudgetExceeded,
}

/// Escape edge counts of one mode, by reason.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct EscapeCounts {
    /// Edges whose closure crossed a predicate.
    pub predicate: usize,
    /// Edges whose closure accumulated a position-dependent action.
    pub custom_action: usize,
    /// Edges whose closed configs have a recursive context.
    pub recursive_context: usize,
    /// Edges past the state budget. They carry no continuation.
    pub budget: usize,
}

/// The escape reason of one edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EscapeReason {
    Predicate,
    CustomAction,
    RecursiveContext,
    Budget,
}

/// Builds static lexer DFA tables from a lexer [`ATN`].
#[derive(Debug, Clone)]
pub struct StaticLexerDfaBuilder {
    max_states_per_mode: usize,
}

impl Default for StaticLexerDfaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StaticLexerDfaBuilder {
    /// A builder with the default per-mode state budget of
    /// [`DEFAULT_MAX_STATES_PER_MODE`].
    pub fn new() -> Self {
        Self {
            max_states_per_mode: DEFAULT_MAX_STATES_PER_MODE,
        }
    }

    /// Sets the per-mode state budget. The budget clamps to the table limit of
    /// 0x7FFE states per mode.
    pub fn with_max_states_per_mode(mut self, max_states_per_mode: usize) -> Self {
        self.max_states_per_mode = max_states_per_mode;
        self
    }

    /// Builds the tables of every lexer mode of `atn`.
    pub fn build(&self, atn: &ATN) -> StaticLexerDfaBuild {
        let num_modes = atn.mode_to_start_state.len();
        let mut modes = Vec::with_capacity(num_modes);
        for mode in 0..num_modes {
            modes.push(self.build_mode(atn, mode));
        }
        let tables = StaticLexerTables::from_modes(
            modes
                .iter_mut()
                .map(|outcome| outcome.table.take())
                .collect(),
        );
        let words = tables.to_words();
        for (outcome, table) in modes.iter_mut().zip(tables.into_modes()) {
            outcome.table = table;
        }
        StaticLexerDfaBuild { words, modes }
    }

    fn build_mode(&self, atn: &ATN, mode_index: usize) -> ModeBuildOutcome {
        // One store and one context cache per mode. The store arena backs
        // both the scratch configs and the finalized configs of the mode
        // build, so the store must outlive the `run_mode_build` call.
        let store = BuildStore::new();
        let cache = PredictionContextCache::new();
        // SAFETY: `store` and `cache` are stack locals of this function. Both
        // are declared before the `run_mode_build` call, neither moves, and
        // both drop only after the call returns, so they outlive every `&'sc`
        // reference that `fin` hands out. The scoped `'sc` parameter of
        // `run_mode_build` binds the fabricated references to the call scope:
        // inside the callee `'sc` is opaque and cannot unify with `'static`,
        // so no finalized config or interned reference can escape the scope.
        let fin = Finalizer {
            cache: unsafe {
                std::mem::transmute::<&PredictionContextCache<'static>, &PredictionContextCache>(
                    &cache,
                )
            },
            store: unsafe { std::mem::transmute::<&BuildStore<'static>, &BuildStore>(&store) },
        };
        self.run_mode_build(atn, mode_index, fin)
    }

    /// Runs the build of one mode. The `'sc` lifetime ties the finalized
    /// configs to the store and cache of the caller. It is opaque in this
    /// body, so nothing that borrows from them can escape the call.
    fn run_mode_build<'sc>(
        &self,
        atn: &ATN,
        mode_index: usize,
        fin: Finalizer<'sc>,
    ) -> ModeBuildOutcome {
        let start_state = atn.mode_to_start_state[mode_index];
        let recursive_rules = detect_simple_recursive_rules(atn, start_state);
        let mut mode = ModeBuild::new(
            self.max_states_per_mode.min(MAX_TABLE_STATE_COUNT),
            recursive_rules,
        );
        let mut env = BuildEnv::default();
        let start_set = start_state_core(atn, start_state.as_ref(), &mut env, fin.store.arena());
        let start_configs = fin.finalize_all(start_set.get_items().cloned());
        let failure = if env.predicate_crossed {
            Some(ModeFailure::StartClosurePredicate)
        } else if has_position_dependent_action(&start_configs) {
            Some(ModeFailure::StartClosureCustomAction)
        } else if start_configs
            .iter()
            .any(|c| has_recursive_context(c.get_context()))
        {
            Some(ModeFailure::StartClosureRecursiveContext)
        } else {
            None
        };
        if let Some(failure) = failure {
            return ModeBuildOutcome::absent(failure);
        }
        if intern(&mut mode, atn, start_configs).is_none() {
            return ModeBuildOutcome::absent(ModeFailure::StartStateBudgetExceeded);
        }

        let mut cursor = 0;
        while cursor < mode.states.len() {
            let source = mode.states[cursor].configs.clone();
            let rows = compute_state_rows(&mut mode, atn, fin, &source);
            mode.rows.push(rows);
            cursor += 1;
        }

        let num_states = mode.states.len();
        let num_continuations = mode.continuations.len();
        let escape_counts = mode.counts;
        ModeBuildOutcome {
            table: Some(mode.finalize(atn)),
            failure: None,
            num_states,
            num_continuations,
            escape_counts,
        }
    }
}

impl ModeBuildOutcome {
    fn absent(failure: ModeFailure) -> Self {
        ModeBuildOutcome {
            table: None,
            failure: Some(failure),
            num_states: 0,
            num_continuations: 0,
            escape_counts: EscapeCounts::default(),
        }
    }
}

/// The build-time [`LexerClosureEnv`]: no live input, predicates evaluate as
/// `true` and flag the edge.
#[derive(Debug, Default)]
struct BuildEnv {
    predicate_crossed: bool,
}

impl LexerClosureEnv for BuildEnv {
    fn input_index(&mut self) -> isize {
        0
    }

    fn eval_predicate(&mut self, _rule_index: i32, _pred_index: i32, _speculative: bool) -> bool {
        self.predicate_crossed = true;
        true
    }
}

/// The build-side [`FinalizeStore`]
///
/// A bump arena that backs both the scratch and the finalized configs of one
/// mode, plus interning maps for lexer action executors and semantic contexts.
/// One store per mode: the arena must live exactly as long as the [`ModeBuild`]
/// that borrows it.
#[derive(Debug)]
struct BuildStore<'sc> {
    executors: RefCell<HashSet<&'sc LexerActionExecutor<'sc>>>,
    semantic_contexts: RefCell<HashSet<&'sc SemanticContext<'sc>>>,
    // Drop order: fields drop in declaration order, and `arena` backs the
    // references in the sets above, so `arena` must stay the last field.
    arena: Box<bumpalo::Bump>,
}

impl<'sc> BuildStore<'sc> {
    fn new() -> Self {
        BuildStore {
            executors: RefCell::new(HashSet::new()),
            semantic_contexts: RefCell::new(HashSet::new()),
            arena: Box::new(bumpalo::Bump::new()),
        }
    }

    fn arena(&self) -> &'sc bumpalo::Bump {
        // SAFETY: self-reference cast, the same pattern as `DFAStateStore`.
        // The caller keeps the store alive as long as the mode build that
        // holds the handed-out references.
        unsafe { std::mem::transmute::<&bumpalo::Bump, &'sc bumpalo::Bump>(&self.arena) }
    }
}

impl<'sc> FinalizeStore<'sc> for BuildStore<'sc> {
    fn alloc_semantic_context(
        &self,
        f: impl FnOnce() -> SemanticContext<'sc>,
    ) -> &'sc SemanticContext<'sc> {
        let value = f();
        let mut interned = self.semantic_contexts.borrow_mut();
        if let Some(&existing) = interned.get(&value) {
            return existing;
        }
        let reference = self.arena().alloc(value);
        interned.insert(reference);
        reference
    }

    fn contains_semantic_context_slice(&self, slice: &[SemanticContext]) -> bool {
        is_slice_in_arena(slice, self.arena())
    }

    fn alloc_semantic_context_slice<I>(&self, iter: I) -> &'sc [SemanticContext<'sc>]
    where
        I: IntoIterator<Item = SemanticContext<'sc>>,
        I::IntoIter: ExactSizeIterator,
    {
        self.arena().alloc_slice_fill_iter(iter)
    }

    fn alloc_lexer_action_slice(
        &self,
        len: usize,
        f: impl FnMut(usize) -> LexerAction<'sc>,
    ) -> &'sc [LexerAction<'sc>] {
        self.arena().alloc_slice_fill_with(len, f)
    }

    fn contains_lexer_action_slice(&self, slice: &[LexerAction]) -> bool {
        is_slice_in_arena(slice, self.arena())
    }

    fn alloc_lexer_action_executor(
        &self,
        f: impl FnOnce() -> LexerActionExecutor<'sc>,
    ) -> &'sc LexerActionExecutor<'sc> {
        let value = f();
        let mut interned = self.executors.borrow_mut();
        if let Some(&existing) = interned.get(&value) {
            return existing;
        }
        let reference = self.arena().alloc(value);
        interned.insert(reference);
        reference
    }

    fn alloc_lexer_indexed_custom_action(
        &self,
        f: impl FnOnce() -> LexerIndexedCustomAction<'sc>,
    ) -> &'sc LexerIndexedCustomAction<'sc> {
        self.arena().alloc(f())
    }

    fn contains_lexer_indexed_custom_action(&self, action: &LexerIndexedCustomAction) -> bool {
        is_ref_in_arena(action, self.arena())
    }

    fn alloc_config_slice<AC, I>(&self, iter: I) -> &'sc [AC]
    where
        I: IntoIterator<Item = AC>,
        I::IntoIter: ExactSizeIterator,
    {
        self.arena().alloc_slice_fill_iter(iter)
    }
}

#[derive(Clone, Copy)]
struct Finalizer<'sc> {
    cache: &'sc PredictionContextCache<'sc>,
    store: &'sc BuildStore<'sc>,
}

impl<'sc> Finalizer<'sc> {
    fn finalize_all<'a>(
        &self,
        configs: impl IntoIterator<Item = LexerATNConfig<'a>>,
    ) -> Vec<LexerATNConfig<'sc>> {
        configs
            .into_iter()
            .map(|c| c.finalize(self.cache, self.store))
            .collect()
    }

    fn finalize_pairs<'a>(
        &self,
        configs: impl IntoIterator<Item = (LexerATNConfig<'a>, bool)>,
    ) -> Vec<(LexerATNConfig<'sc>, bool)> {
        configs
            .into_iter()
            .map(|(c, flag)| (c.finalize(self.cache, self.store), flag))
            .collect()
    }
}

/// True when a config of the set accumulated a position-dependent (custom)
/// lexer action.
fn has_position_dependent_action(configs: &[LexerATNConfig<'_>]) -> bool {
    configs.iter().any(|c| {
        c.get_lexer_executor()
            .is_some_and(|ex| ex.actions().iter().any(LexerAction::is_position_dependent))
    })
}

/// True when the context chain has a repeated return state or is deeper than
/// [`MAX_CONTEXT_DEPTH`]. Lexer contexts are empty or singleton chains.
fn has_recursive_context(mut ctx: Option<PredictionContextRef>) -> bool {
    let mut seen = HashSet::new();
    let mut depth = 0;
    while let Some(c) = ctx {
        if c.is_empty() {
            break;
        }
        assert_eq!(
            c.length(),
            1,
            "lexer prediction contexts are singleton chains"
        );
        depth += 1;
        if depth > MAX_CONTEXT_DEPTH || !seen.insert(c.get_return_state(0).get_state_number()) {
            return true;
        }
        ctx = c.get_parent(0);
    }
    false
}

/// The ATN indices of the actions of the executor of a config. Fails on a
/// position-dependent action.
fn config_action_indices(atn: &ATN, config: &LexerATNConfig<'_>) -> Result<Vec<u32>, ()> {
    let mut actions = Vec::new();
    if let Some(executor) = config.get_lexer_executor() {
        for action in executor.actions() {
            if action.is_position_dependent() {
                return Err(());
            }
            let index = atn
                .lexer_actions
                .iter()
                .position(|a| a == action)
                .expect("config actions come from the ATN lexer action list");
            actions.push(index as u32);
        }
    }
    Ok(actions)
}

/// One finalized DFA state: the finalized closed config set in insertion order
/// and the accept data. `accept_actions` holds the ATN indices of the actions
/// of the accept config.
#[derive(Debug)]
struct StateData<'sc> {
    configs: Vec<LexerATNConfig<'sc>>,
    accept_type: i32,
    accept_actions: Vec<u32>,
}

/// The structured target of one DFA edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeEntry {
    /// The edge does not exist.
    None,
    /// The edge targets the table state with the given id.
    Target(u32),
    /// The edge can not be fully resolved by the static DFA builder and thus
    /// must be deferred to the runtime ATN simulator: `Some` names the
    /// continuation pool index, `None` is a budget escape and carries no
    /// continuation.
    Escape(Option<u32>),
}

/// One edge interval of a state. Covers `lo..=hi`, both inclusive.
#[derive(Debug, Clone, Copy)]
struct EdgeInterval {
    lo: i32,
    hi: i32,
    entry: EdgeEntry,
}

/// The edges of one state: the EOF edge and the sorted, disjoint symbol
/// intervals that have an edge.
#[derive(Debug)]
struct StateRows {
    eof: EdgeEntry,
    intervals: Vec<EdgeInterval>,
}

/// The kind of a [`PendingContinuation`]. The tag values come from
/// [`ContinuationKind::tag`] at commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ContinuationKind {
    /// The post-move, pre-closure configs of the escape edge.
    MovedConfigs,
    /// The source configs of the state. The resume re-runs the full move
    /// step, because the build-time move pruned a config.
    SourceSet,
}

impl ContinuationKind {
    fn tag(self) -> u32 {
        match self {
            ContinuationKind::MovedConfigs => CONTINUATION_MOVED_CONFIGS,
            ContinuationKind::SourceSet => CONTINUATION_SOURCE_SET,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PendingContinuation<'sc> {
    kind: ContinuationKind,
    configs: Vec<(LexerATNConfig<'sc>, bool)>,
}

/// The in-progress build of one mode. The `'sc` lifetime ties the finalized
/// configs to the [`BuildStore`] of the mode.
#[derive(Debug)]
struct ModeBuild<'sc> {
    states: Vec<StateData<'sc>>,
    rows: Vec<StateRows>,
    ids: HashMap<Vec<LexerATNConfig<'sc>>, u32>,
    continuations: Vec<PendingContinuation<'sc>>,
    cont_ids: HashMap<PendingContinuation<'sc>, u32>,
    counts: EscapeCounts,
    max_states: usize,
    recursive_rules: Vec<SimpleRecursiveRule>,
}

impl<'sc> ModeBuild<'sc> {
    fn new(max_states: usize, recursive_rules: Vec<SimpleRecursiveRule>) -> Self {
        ModeBuild {
            states: Vec::new(),
            rows: Vec::new(),
            ids: HashMap::new(),
            continuations: Vec::new(),
            cont_ids: HashMap::new(),
            counts: EscapeCounts::default(),
            max_states,
            recursive_rules,
        }
    }

    fn count_escape(&mut self, reason: EscapeReason) {
        match reason {
            EscapeReason::Predicate => self.counts.predicate += 1,
            EscapeReason::CustomAction => self.counts.custom_action += 1,
            EscapeReason::RecursiveContext => self.counts.recursive_context += 1,
            EscapeReason::Budget => self.counts.budget += 1,
        }
    }

    /// Dedupes a continuation by structural content and returns its pool
    /// index.
    fn intern_continuation(&mut self, cont: PendingContinuation<'sc>) -> u32 {
        if let Some(&id) = self.cont_ids.get(&cont) {
            return id;
        }
        let id = self.continuations.len() as u32;
        self.cont_ids.insert(cont.clone(), id);
        self.continuations.push(cont);
        id
    }

    /// Converts the structural build into a final [`StaticLexerDFA`].
    fn finalize(self, atn: &ATN) -> StaticLexerDFA {
        let num_states = self.states.len();
        let accept_type: Vec<i32> = self.states.iter().map(|s| s.accept_type).collect();

        // Low rows: column 0 is the EOF edge, columns 1..=128 are the
        // symbols 0..=127.
        let mut low = vec![NO_EDGE; num_states * LOW_ROW];
        for (s, rows) in self.rows.iter().enumerate() {
            low[s * LOW_ROW] = pack_entry(&accept_type, rows.eof);
            for iv in &rows.intervals {
                if iv.lo <= ASCII_MAX {
                    let entry = pack_entry(&accept_type, iv.entry);
                    for c in iv.lo..=iv.hi.min(ASCII_MAX) {
                        low[s * LOW_ROW + 1 + c as usize] = entry;
                    }
                }
            }
        }

        // Hi-class compression: the class bounds are the union of the
        // hi-edge bounds of the mode; rows dedupe by content; `class_of`
        // derives from the class starts (the loader's invariant).
        let class_bounds = collect_class_starts(&self.rows);
        let num_classes = class_bounds.len();
        let mut class_starts = Vec::new();
        let mut class_of = Vec::new();
        let mut hi_rows = Vec::new();
        let mut row_of = Vec::new();
        if num_classes > 0 {
            class_starts = class_bounds.iter().map(|&s| s as u32).collect();
            class_of = class_map(&class_starts);
            let mut rows: Vec<Vec<(u32, u32, u16)>> = Vec::new();
            let mut row_ids: HashMap<Vec<(u32, u32, u16)>, u32> = HashMap::new();
            row_of = Vec::with_capacity(num_states);
            for state_rows in &self.rows {
                let runs = class_runs(&class_bounds, &hi_clipped(&accept_type, state_rows));
                let id = *row_ids.entry(runs).or_insert_with_key(|runs| {
                    rows.push(runs.clone());
                    (rows.len() - 1) as u32
                });
                row_of.push(id);
            }
            hi_rows = dense_hi_rows(&rows, num_classes);
        }

        // Accept action tables: `LexerAction` clones from the ATN, the
        // lists deduplicated by content (the loader's invariant).
        let mut actions: Vec<LexerAction<'static>> = Vec::new();
        let mut list_ids: HashMap<Vec<LexerAction<'static>>, (u32, u32)> = HashMap::new();
        let mut accept_actions = Vec::with_capacity(num_states);
        for state in &self.states {
            if state.accept_actions.is_empty() {
                accept_actions.push((0, 0));
                continue;
            }
            let list: Vec<LexerAction<'static>> = state
                .accept_actions
                .iter()
                .map(|&index| atn.lexer_actions[index as usize].clone())
                .collect();
            let entry = list_ids.entry(list).or_insert_with_key(|list| {
                let start = actions.len() as u32;
                actions.extend(list.iter().cloned());
                (start, list.len() as u32)
            });
            accept_actions.push(*entry);
        }

        // Run acceleration over the structural rows.
        let mut accel_tag = vec![0u8; num_states];
        let mut accel_data = Vec::new();
        for (s, rows) in self.rows.iter().enumerate() {
            let accel = maybe_counted_accel(s as u32, rows, &self.states, &self.recursive_rules)
                .or_else(|| compute_accel(s as u32, rows));
            if let Some(accel) = accel {
                accel_data.push(accel);
                assert!(
                    accel_data.len() <= u8::MAX as usize,
                    "a mode has at most {} accel payloads (the tag is a u8)",
                    u8::MAX
                );
                accel_tag[s] = accel_data.len() as u8;
            }
        }

        // Escape ranges: the sorted, disjoint intervals whose edge escapes.
        let mut eof_escape = Vec::with_capacity(num_states);
        let mut escape_off = Vec::with_capacity(num_states + 1);
        let mut escape_ranges = Vec::new();
        escape_off.push(0);
        for rows in &self.rows {
            eof_escape.push(continuation_of(rows.eof));
            for iv in &rows.intervals {
                if let EdgeEntry::Escape(cont) = iv.entry {
                    escape_ranges.push(EscapeRange {
                        lo: iv.lo,
                        hi: iv.hi,
                        continuation: cont.unwrap_or(NO_CONTINUATION),
                    });
                }
            }
            escape_off.push(escape_ranges.len() as u32);
        }

        let continuations = self
            .continuations
            .iter()
            .map(|cont| commit_continuation(atn, cont))
            .collect();

        StaticLexerDFA {
            start_state: 0,
            low,
            class_of,
            hi_rows,
            row_of,
            num_classes,
            class_starts,
            accept_type,
            accept_actions,
            actions,
            accel_tag,
            accel_data,
            eof_escape,
            escape_off,
            escape_ranges,
            continuations,
        }
    }
}

/// Special case to handle rules of the following shape:
///
///   COMMENT : '/*' (COMMENT | .)*? '*/' ;
///
/// instead of deferring these to adaptive runtime, we compile them into a
/// special [`Accel::CountedExclusion`] that uses a depth counter to track the
/// recursion while still utilizing SIMD accelerated scans.
#[derive(Debug, Clone, Copy)]
struct SimpleRecursiveRule {
    rule_index: i32,
    rec_return_state: i32,
    wildcard_state: i32,
    accel: Accel,
}

fn detect_simple_recursive_rules(atn: &ATN, mode_start: ATNStateRef) -> Vec<SimpleRecursiveRule> {
    let mut rules = Vec::new();
    for tr in mode_start.get_transitions() {
        let rule_index = tr.get_target().get_rule_index();
        if rule_index >= 0 {
            if let Some(cc) = detect_simple_recursive_rule(atn, rule_index as usize) {
                rules.push(cc);
            }
        }
    }
    rules
}

fn detect_simple_recursive_rule(atn: &ATN, rule_index: usize) -> Option<SimpleRecursiveRule> {
    use crate::atn_state::ATNStateExtTrait;
    use crate::transition::ConcreteTransition;

    let start_state = *atn.rule_to_start_state.get(rule_index)?;
    let stop_state = *atn.rule_to_stop_state.get(rule_index)?;

    let mut opener = Vec::with_capacity(3);
    let mut cur = start_state;
    let mut steps = 0usize;
    let loop_entry = loop {
        steps += 1;
        if steps > 32 {
            return None;
        }
        if cur.state_type() == ATNStateType::StarLoopEntry {
            break cur;
        }
        let [tr] = cur.get_transitions() else {
            return None;
        };
        match tr.transition_type() {
            TransitionType::Epsilon => {
                cur = tr.get_target();
            }
            TransitionType::Atom => {
                let ch = AtomTransition::cast_from(tr)?.label();
                if !(0..=ASCII_MAX).contains(&ch) || opener.len() >= 3 {
                    return None;
                }
                opener.push(ch as u8);
                cur = tr.get_target();
            }
            _ => return None,
        }
    };
    if opener.is_empty() {
        return None;
    }

    let sle = StarLoopEntryState::try_cast(loop_entry.as_ref())?;
    if !sle.nongreedy {
        return None;
    }
    let [t0, t1] = loop_entry.get_transitions() else {
        return None;
    };
    if !matches!(t0.transition_type(), TransitionType::Epsilon)
        || !matches!(t1.transition_type(), TransitionType::Epsilon)
    {
        return None;
    }
    let (block_start, loop_end) = match (t0.get_target().state_type(), t1.get_target().state_type())
    {
        (ATNStateType::LoopEnd, ATNStateType::StarBlockStart) => (t1.get_target(), t0.get_target()),
        (ATNStateType::StarBlockStart, ATNStateType::LoopEnd) => (t0.get_target(), t1.get_target()),
        _ => return None,
    };

    let [b0, b1] = block_start.get_transitions() else {
        return None;
    };
    if !matches!(b0.transition_type(), TransitionType::Epsilon)
        || !matches!(b1.transition_type(), TransitionType::Epsilon)
    {
        return None;
    }
    let s0 = chase_epsilons(b0.get_target())?;
    let s1 = chase_epsilons(b1.get_target())?;
    let ([tr0], [tr1]) = (s0.get_transitions(), s1.get_transitions()) else {
        return None;
    };
    let (rule_tr, wild_state, wild_tr) = match (tr0.transition_type(), tr1.transition_type()) {
        (TransitionType::Rule, TransitionType::Wildcard) => (tr0, s1, tr1),
        (TransitionType::Wildcard, TransitionType::Rule) => (tr1, s0, tr0),
        _ => return None,
    };
    let rt = RuleTransition::cast_from(rule_tr)?;
    if rt.get_target() != start_state {
        return None;
    }
    if chase_epsilons(rt.follow_state)? != loop_entry {
        return None;
    }
    if chase_epsilons(wild_tr.get_target())? != loop_entry {
        return None;
    }

    let mut closer = Vec::with_capacity(3);
    let mut cur = loop_end;
    let mut seen_action = false;
    let mut steps = 0usize;
    while cur != stop_state {
        steps += 1;
        if steps > 32 {
            return None;
        }
        let [tr] = cur.get_transitions() else {
            return None;
        };
        match tr.transition_type() {
            TransitionType::Epsilon => {
                cur = tr.get_target();
            }
            TransitionType::Atom => {
                if seen_action {
                    return None;
                }
                let ch = AtomTransition::cast_from(tr)?.label();
                if !(0..=ASCII_MAX).contains(&ch) || closer.len() >= 3 {
                    return None;
                }
                closer.push(ch as u8);
                cur = tr.get_target();
            }
            TransitionType::Action => {
                let at = ActionTransition::cast_from(tr)?;
                let action = atn.lexer_actions.get(at.action_index() as usize)?;
                if action.is_position_dependent() {
                    return None;
                }
                seen_action = true;
                cur = tr.get_target();
            }
            _ => return None,
        }
    }

    if closer.len() != opener.len() || closer == opener || !(1..=3).contains(&opener.len()) {
        return None;
    }
    let mut opener_arr = [0u8; 3];
    let mut closer_arr = [0u8; 3];
    opener_arr[..opener.len()].copy_from_slice(&opener);
    closer_arr[..closer.len()].copy_from_slice(&closer);
    let accel = Accel::CountedExclusion {
        opener: opener_arr,
        closer: closer_arr,
        n: opener.len() as u8,
    };
    Some(SimpleRecursiveRule {
        rule_index: rule_index as i32,
        rec_return_state: rt.follow_state.get_state_number(),
        wildcard_state: wild_state.get_state_number(),
        accel,
    })
}

fn chase_epsilons(mut state: ATNStateRef) -> Option<ATNStateRef> {
    for _ in 0..16 {
        let [tr] = state.get_transitions() else {
            return Some(state);
        };
        if !matches!(tr.transition_type(), TransitionType::Epsilon) {
            return Some(state);
        }
        state = tr.get_target();
    }
    None
}

fn context_has_return_state(mut ctx: Option<PredictionContextRef>, return_state: i32) -> bool {
    while let Some(c) = ctx {
        if c.is_empty() {
            break;
        }
        if c.get_return_state(0).get_state_number() == return_state {
            return true;
        }
        ctx = c.get_parent(0);
    }
    false
}

fn is_simple_recursive_body(configs: &[LexerATNConfig<'_>], cc: &SimpleRecursiveRule) -> bool {
    !configs.is_empty()
        && configs.iter().all(|c| {
            let s = c.get_state();
            if s.get_rule_index() == cc.rule_index {
                s.state_type() != ATNStateType::RuleStop
            } else {
                s.state_type() == ATNStateType::RuleStop
                    && c.get_context().is_none_or(|ctx| ctx.is_empty())
            }
        })
        && configs.iter().any(|c| {
            c.get_state().get_state_number() == cc.wildcard_state
                && c.get_context().is_none_or(|ctx| ctx.is_empty())
        })
}

fn strip_simple_recursion(
    closed: &mut Vec<LexerATNConfig<'_>>,
    simple_recursive_rules: &[SimpleRecursiveRule],
) {
    if closed.is_empty() || simple_recursive_rules.is_empty() {
        return;
    }
    for cc in simple_recursive_rules {
        if is_simple_recursive_body(closed, cc) {
            closed.retain(|c| !context_has_return_state(c.get_context(), cc.rec_return_state));
            return;
        }
    }
}

fn maybe_counted_accel(
    state: u32,
    rows: &StateRows,
    states: &[StateData<'_>],
    recursive_rules: &[SimpleRecursiveRule],
) -> Option<Accel> {
    let state_data = &states[state as usize];
    if state_data.configs.is_empty() || recursive_rules.is_empty() {
        return None;
    }
    let cc = recursive_rules
        .iter()
        .find(|cc| is_simple_recursive_body(&state_data.configs, cc))?;
    let mut hi_covered_to = ASCII_MAX;
    let mut hi_target = None;
    for iv in &rows.intervals {
        if iv.hi > ASCII_MAX && iv.lo <= hi_covered_to + 1 {
            let EdgeEntry::Target(target) = iv.entry else {
                break;
            };
            match hi_target {
                None => hi_target = Some(target),
                Some(t) if t == target => {}
                Some(_) => break,
            }
            hi_covered_to = hi_covered_to.max(iv.hi);
        }
    }
    let hi_target = hi_target?;
    if hi_covered_to < MAX_SYMBOL - 1 {
        return None;
    }
    if hi_target != state {
        let target_configs = &states[hi_target as usize].configs;
        let non_stop = state_data
            .configs
            .iter()
            .filter(|c| c.get_state().state_type() != ATNStateType::RuleStop);
        if !non_stop.eq(target_configs.iter()) {
            return None;
        }
    }
    Some(cc.accel)
}

/// The run-acceleration descriptor of one state, classified from its structural
/// edges: the intervals whose entry loops back to the state. `None` when the
/// state has no worthwhile run.
fn compute_accel(state: u32, rows: &StateRows) -> Option<Accel> {
    let mut bitmap = 0u128;
    let mut hi_covered_to = ASCII_MAX;
    for iv in &rows.intervals {
        if !matches!(iv.entry, EdgeEntry::Target(target) if target == state) {
            continue;
        }
        if iv.lo <= ASCII_MAX {
            let lo = iv.lo.max(0) as u32;
            let hi = iv.hi.min(ASCII_MAX) as u32;
            let len = hi - lo + 1;
            bitmap |= (u128::MAX >> (128 - len)) << lo;
        }
        if iv.hi > ASCII_MAX && iv.lo <= hi_covered_to + 1 {
            hi_covered_to = hi_covered_to.max(iv.hi);
        }
    }
    if bitmap == 0 {
        return None;
    }

    // The exclusion flavor requires every symbol above 127 to loop, all the
    // way to MAX_SYMBOL.
    if hi_covered_to == MAX_SYMBOL {
        let mut excluded = !bitmap;
        match excluded.count_ones() {
            1 => return Some(Accel::Exclude(excluded.trailing_zeros() as u8)),
            2 => {
                let n1 = excluded.trailing_zeros() as u8;
                excluded &= excluded - 1;
                let n2 = excluded.trailing_zeros() as u8;
                return Some(Accel::Exclude2(n1, n2));
            }
            3 => {
                let n1 = excluded.trailing_zeros() as u8;
                excluded &= excluded - 1;
                let n2 = excluded.trailing_zeros() as u8;
                excluded &= excluded - 1;
                let n3 = excluded.trailing_zeros() as u8;
                return Some(Accel::Exclude3(n1, n2, n3));
            }
            _ => (),
        }
    }

    Some(Accel::Positive(bitmap))
}

/// Runs move+closure for one edge of a state, finalizes the closed configs
/// into the build store, and classifies the edge.
fn compute_edge<'sc>(
    mode: &mut ModeBuild<'sc>,
    atn: &ATN,
    fin: Finalizer<'sc>,
    source: &[LexerATNConfig<'sc>],
    t: i32,
) -> EdgeEntry {
    let mut env = BuildEnv::default();
    let mut reach = MutableLexerATNConfigSet::new(fin.store.arena());
    let mut capture = ReachCapture::default();
    reachable_core(
        atn,
        0,
        source.iter(),
        &mut reach,
        t,
        &mut env,
        Some(&mut capture),
        fin.store.arena(),
    );
    let mut closed = fin.finalize_all(reach.get_items().cloned());
    strip_simple_recursion(&mut closed, &mode.recursive_rules);
    let reason = if env.predicate_crossed {
        Some(EscapeReason::Predicate)
    } else if has_position_dependent_action(&closed) {
        Some(EscapeReason::CustomAction)
    } else if closed
        .iter()
        .any(|c| has_recursive_context(c.get_context()))
    {
        Some(EscapeReason::RecursiveContext)
    } else {
        None
    };

    if let Some(reason) = reason {
        // An empty build-time reach is a real no-edge: with no pruning
        // activity, the runtime closure is a subset of the build-time closure
        // (predicates can only remove configs), so it stays empty. With
        // pruning activity, pruned configs can move at runtime: escape.
        if closed.is_empty() && !capture.pruning_activity {
            return EdgeEntry::None;
        }
        mode.count_escape(reason);
        let cont = if capture.pruning_activity {
            mode.intern_continuation(PendingContinuation {
                kind: ContinuationKind::SourceSet,
                configs: source.iter().map(|c| (c.clone(), false)).collect(),
            })
        } else {
            mode.intern_continuation(PendingContinuation {
                kind: ContinuationKind::MovedConfigs,
                configs: fin
                    .finalize_pairs(capture.moved.iter().map(|(c, flag)| (c.clone(), *flag))),
            })
        };
        return EdgeEntry::Escape(Some(cont));
    }

    if closed.is_empty() {
        return EdgeEntry::None;
    }
    match intern(mode, atn, closed) {
        Some(target) => EdgeEntry::Target(target),
        None => {
            mode.count_escape(EscapeReason::Budget);
            EdgeEntry::Escape(None)
        }
    }
}

/// The accept data of a closed config set: the first rule-stop config in
/// insertion order decides, mirroring `DFA::add_lexer_state`.
fn accept_of(atn: &ATN, configs: &[LexerATNConfig<'_>]) -> (i32, Vec<u32>) {
    let Some(accept) = configs
        .iter()
        .find(|c| c.get_state().state_type() == ATNStateType::RuleStop)
    else {
        return (-1, Vec::new());
    };
    let rule = accept.get_state().get_rule_index();
    let accept_type = atn.rule_to_token_type[rule as usize];
    let actions = config_action_indices(atn, accept)
        .expect("interned states hold no position-dependent actions");
    (accept_type, actions)
}

/// Interns a finalized closed config set as a table state. Returns `None`
/// when the state budget is spent.
fn intern<'sc>(
    mode: &mut ModeBuild<'sc>,
    atn: &ATN,
    configs: Vec<LexerATNConfig<'sc>>,
) -> Option<u32> {
    if let Some(&id) = mode.ids.get(configs.as_slice()) {
        return Some(id);
    }
    if mode.states.len() >= mode.max_states {
        return None;
    }
    let id = mode.states.len() as u32;
    let (accept_type, accept_actions) = accept_of(atn, &configs);
    mode.ids.insert(configs.clone(), id);
    mode.states.push(StateData {
        configs,
        accept_type,
        accept_actions,
    });
    Some(id)
}

/// Adds the cut points of the match set of one transition to `bounds`. A
/// target is constant between two consecutive cut points.
fn add_transition_bounds(tr: &Transition, bounds: &mut BTreeSet<i32>) {
    match tr.transition_type() {
        TransitionType::Atom | TransitionType::Range | TransitionType::Set => {
            if let Some(set) = tr.get_label() {
                for iv in set.as_slice() {
                    add_interval_bounds(iv.a, iv.b, bounds);
                }
            }
        }
        TransitionType::NotSet => {
            // The complement has the same cut points as the set itself.
            if let Some(set) = tr.get_label() {
                for iv in set.as_slice() {
                    add_interval_bounds(iv.a, iv.b, bounds);
                }
            }
        }
        TransitionType::Wildcard => {
            // Wildcard matches strictly inside the vocabulary bounds.
            bounds.insert(1);
            bounds.insert(MAX_SYMBOL);
        }
        _ => {}
    }
}

fn add_interval_bounds(a: i32, b: i32, bounds: &mut BTreeSet<i32>) {
    if b < 0 || a > MAX_SYMBOL {
        return;
    }
    bounds.insert(a.max(0));
    if b < MAX_SYMBOL {
        bounds.insert(b + 1);
    }
}

/// Computes every edge of one state: the EOF edge, then one edge per symbol
/// interval between consecutive cut points.
fn compute_state_rows<'sc>(
    mode: &mut ModeBuild<'sc>,
    atn: &ATN,
    fin: Finalizer<'sc>,
    source: &[LexerATNConfig<'sc>],
) -> StateRows {
    let eof = compute_edge(mode, atn, fin, source, EOF);
    let mut bounds = BTreeSet::from([0, MAX_SYMBOL + 1]);
    for config in source {
        for tr in config.get_state().get_transitions() {
            add_transition_bounds(tr, &mut bounds);
        }
    }
    let mut intervals: Vec<EdgeInterval> = Vec::new();
    let mut lo = 0;
    for b in bounds {
        if lo < b {
            let hi = b - 1;
            let edge = compute_edge(mode, atn, fin, source, lo);
            if edge == EdgeEntry::None {
                lo = b;
                continue;
            }
            if let Some(last) = intervals.last_mut() {
                if last.entry == edge && last.hi + 1 == lo {
                    last.hi = hi;
                    lo = b;
                    continue;
                }
            }
            intervals.push(EdgeInterval {
                lo,
                hi,
                entry: edge,
            });
        }
        lo = b;
    }
    StateRows { eof, intervals }
}

/// The packed table entry of a structured edge: the target id or-ed with
/// the `ACCEPT_BIT` of the target state, `NO_EDGE`, or `ESCAPE`.
fn pack_entry(accept_type: &[i32], entry: EdgeEntry) -> u16 {
    match entry {
        EdgeEntry::None => NO_EDGE,
        EdgeEntry::Target(target) => {
            let mut packed = target as u16;
            if accept_type[target as usize] >= 0 {
                packed |= ACCEPT_BIT;
            }
            packed
        }
        EdgeEntry::Escape(_) => ESCAPE,
    }
}

/// The continuation pool index of an escape edge, or `NO_CONTINUATION`.
fn continuation_of(entry: EdgeEntry) -> u32 {
    match entry {
        EdgeEntry::Escape(Some(cont)) => cont,
        _ => NO_CONTINUATION,
    }
}

/// The edges of a state for the symbols above 127, clipped to start at 128
/// and packed to table entries.
fn hi_clipped(accept_type: &[i32], rows: &StateRows) -> Vec<(i32, i32, u16)> {
    rows.intervals
        .iter()
        .filter(|iv| iv.hi > ASCII_MAX)
        .map(|iv| {
            (
                iv.lo.max(ASCII_MAX + 1),
                iv.hi,
                pack_entry(accept_type, iv.entry),
            )
        })
        .collect()
}

/// The class bounds of a mode: the union of the hi-edge bounds of every
/// state. Empty when the mode has no edges above 127.
fn collect_class_starts(rows: &[StateRows]) -> Vec<i32> {
    let mut bounds = BTreeSet::new();
    for r in rows {
        for iv in &r.intervals {
            if iv.hi > ASCII_MAX {
                bounds.insert(iv.lo.max(ASCII_MAX + 1));
                if iv.hi < MAX_SYMBOL {
                    bounds.insert(iv.hi + 1);
                }
            }
        }
    }
    if bounds.is_empty() {
        return Vec::new();
    }
    bounds.insert(ASCII_MAX + 1);
    bounds.into_iter().collect()
}

/// The class of a symbol: the index of the last class start at or below it.
fn class_index(class_starts: &[i32], symbol: i32) -> u32 {
    class_starts.partition_point(|&s| s <= symbol) as u32 - 1
}

/// The deduplicated class row of one state: `(lo_class, hi_class, entry)`
/// runs, with adjacent runs of the same entry merged.
fn class_runs(class_starts: &[i32], hi: &[(i32, i32, u16)]) -> Vec<(u32, u32, u16)> {
    let mut runs: Vec<(u32, u32, u16)> = Vec::new();
    for &(lo, hi, entry) in hi {
        let lo_class = class_index(class_starts, lo);
        let hi_class = class_index(class_starts, hi);
        if let Some(last) = runs.last_mut() {
            if last.2 == entry && last.1 + 1 == lo_class {
                last.1 = hi_class;
                continue;
            }
        }
        runs.push((lo_class, hi_class, entry));
    }
    runs
}

/// Converts a structural continuation to the walker's pool entry: the
/// continuation-local topological context table and the [`ContinuationConfig`]
/// structs. Local id 0 is the empty context. Entry `i` has local id `i + 1`,
/// and parents refer to smaller ids. `(parent_local, return_state)` keys the
/// local dedup map: after finalization the pair names a context node exactly.
fn commit_continuation(atn: &ATN, cont: &PendingContinuation) -> LexerContinuation {
    let mut ctxs: Vec<(u32, i32)> = Vec::new();
    let mut local_ids: HashMap<(u32, i32), u32> = HashMap::new();
    let mut configs = Vec::with_capacity(cont.configs.len());
    for (config, alt_reached_accept) in &cont.configs {
        let mut chain = Vec::new();
        let mut cur = config.get_context();
        while let Some(c) = cur {
            if c.is_empty() {
                break;
            }
            assert_eq!(
                c.length(),
                1,
                "lexer prediction contexts are singleton chains"
            );
            chain.push(c.get_return_state(0).get_state_number());
            cur = c.get_parent(0);
        }
        chain.reverse();
        let mut parent_local = 0;
        for return_state in chain {
            if let Some(&local) = local_ids.get(&(parent_local, return_state)) {
                parent_local = local;
                continue;
            }
            ctxs.push((parent_local, return_state));
            let local = ctxs.len() as u32;
            local_ids.insert((parent_local, return_state), local);
            parent_local = local;
        }
        let actions = config_action_indices(atn, config)
            .expect("escape continuations hold no position-dependent actions");
        configs.push(ContinuationConfig {
            state: config.get_state().get_state_number(),
            alt: config.get_alt(),
            ctx: parent_local,
            non_greedy: config.has_passed_through_non_greedy_decision(),
            alt_reached_accept: *alt_reached_accept,
            actions,
        });
    }
    LexerContinuation {
        kind: cont.kind.tag(),
        ctxs,
        configs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::atn_deserializer::ATNDeserializer;
    use crate::atn_state::ATNStateRef;
    use crate::dfa::static_lexer_dfa::{ACCEL_EXCLUSION, ACCEL_POSITIVE};
    use crate::prediction_context::PredictionContext;

    fn push_ctx<'a>(
        scratch: &'a bumpalo::Bump,
        parent: Option<PredictionContextRef<'a>>,
        return_state: ATNStateRef,
    ) -> PredictionContextRef<'a> {
        PredictionContextRef::from(
            &*scratch.alloc(PredictionContext::new_singleton(parent, return_state)),
        )
    }

    #[test]
    fn recursive_context_detection() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/static_lexer/XMLLexer.g4");
        let units = dbt_antlr_codegen::atn_export::compile_atn_data(&path, &[]).expect("compile");
        let lexer = units
            .iter()
            .find(|u| u.lexer_atn_words.is_some())
            .expect("lexer unit");
        let atn = ATNDeserializer::new(None)
            .deserialize(&mut lexer.lexer_atn_words.as_ref().expect("lexer words").iter());
        let scratch = bumpalo::Bump::new();

        let a = push_ctx(
            &scratch,
            Some(PredictionContextRef::new_empty()),
            atn.make_state_ref(10),
        );
        let b = push_ctx(&scratch, Some(a), atn.make_state_ref(20));
        assert!(!has_recursive_context(Some(b)));
        let c = push_ctx(&scratch, Some(b), atn.make_state_ref(10));
        assert!(has_recursive_context(Some(c)));
        let mut deep = PredictionContextRef::new_empty();
        for i in 0..33 {
            deep = push_ctx(&scratch, Some(deep), atn.make_state_ref(100 + i));
        }
        assert!(has_recursive_context(Some(deep)));
    }

    #[test]
    fn class_starts_cover_hi_edge_bounds() {
        let rows = vec![StateRows {
            eof: EdgeEntry::None,
            intervals: vec![
                EdgeInterval {
                    lo: 65,
                    hi: 200,
                    entry: EdgeEntry::Target(3),
                },
                EdgeInterval {
                    lo: 0x1000,
                    hi: 0x2000,
                    entry: EdgeEntry::Target(4),
                },
            ],
        }];
        let starts = collect_class_starts(&rows);
        assert_eq!(starts, vec![128, 201, 0x1000, 0x2001]);
        assert_eq!(class_index(&starts, 128), 0);
        assert_eq!(class_index(&starts, 200), 0);
        assert_eq!(class_index(&starts, 201), 1);
        assert_eq!(class_index(&starts, 0x2000), 2);
        assert_eq!(class_index(&starts, MAX_SYMBOL), 3);

        let no_hi = vec![StateRows {
            eof: EdgeEntry::None,
            intervals: vec![EdgeInterval {
                lo: 65,
                hi: 90,
                entry: EdgeEntry::Target(3),
            }],
        }];
        assert!(collect_class_starts(&no_hi).is_empty());
    }

    #[test]
    fn class_runs_merge_adjacent_equal_entries() {
        let starts = vec![128, 200, 251, 260];
        let hi = vec![(128, 199, 7u16), (200, 250, 7), (260, 300, 7)];
        let runs = class_runs(&starts, &hi);
        assert_eq!(runs, vec![(0, 1, 7), (3, 3, 7)]);
    }

    /// Builds the structural rows of one state from `(lo, hi, entry)`
    /// intervals.
    fn rows(intervals: Vec<(i32, i32, EdgeEntry)>) -> StateRows {
        StateRows {
            eof: EdgeEntry::None,
            intervals: intervals
                .into_iter()
                .map(|(lo, hi, entry)| EdgeInterval { lo, hi, entry })
                .collect(),
        }
    }

    #[test]
    fn accel_exclusion_needles() {
        let rows = rows(vec![
            (0, i32::from(b'a') - 1, EdgeEntry::Target(2)),
            (i32::from(b'b') + 1, MAX_SYMBOL, EdgeEntry::Target(2)),
        ]);
        let accel = compute_accel(2, &rows).expect("exclusion run");
        assert_eq!(accel, Accel::Exclude2(b'a', b'b'));
        assert_eq!(
            accel.to_serialized(2),
            vec![2, ACCEL_EXCLUSION, 2, u32::from(b'a'), u32::from(b'b'), 0]
        );
    }

    #[test]
    fn accel_positive_bitmap() {
        // State 1 loops on the identifier bytes 'a'..='m'; no hi edges.
        let rows = rows(vec![(
            i32::from(b'a'),
            i32::from(b'm'),
            EdgeEntry::Target(1),
        )]);
        let accel = compute_accel(1, &rows).expect("positive run");
        let Accel::Positive(bitmap) = accel else {
            panic!("expected a positive accel, got {accel:?}");
        };
        assert_eq!(bitmap & (1 << b'\n'), 0);
        assert_eq!(bitmap.count_ones(), 13);
        let words = accel.to_serialized(1);
        assert_eq!(words.len(), 6);
        assert_eq!(words[0], 1);
        assert_eq!(words[1], ACCEL_POSITIVE);
        let mut round_trip = 0u128;
        for i in 0..4 {
            round_trip |= u128::from(words[2 + i]) << (i * 32);
        }
        assert_eq!(round_trip, bitmap);
    }

    #[test]
    fn accel_positive_when_newline_loops() {
        let rows = rows(vec![(0, 20, EdgeEntry::Target(3))]);
        assert_eq!(
            compute_accel(3, &rows),
            Some(Accel::Positive((1u128 << 21) - 1))
        );
    }

    #[test]
    fn accel_none_when_no_ascii_loops() {
        let rows = rows(vec![(128, MAX_SYMBOL, EdgeEntry::Target(3))]);
        assert_eq!(compute_accel(3, &rows), None);
    }

    #[test]
    fn counted_exclude_serialization() {
        use crate::dfa::static_lexer_dfa::ACCEL_COUNTED_EXCLUSION;
        let a1 = Accel::CountedExclusion {
            opener: [b'{', 0, 0],
            closer: [b'}', 0, 0],
            n: 1,
        };
        assert_eq!(
            a1.to_serialized(4),
            vec![
                4,
                ACCEL_COUNTED_EXCLUSION,
                1,
                u32::from(b'{'),
                u32::from(b'}'),
                0
            ]
        );
        let a2 = Accel::CountedExclusion {
            opener: [b'/', b'*', 0],
            closer: [b'*', b'/', 0],
            n: 2,
        };
        assert_eq!(
            a2.to_serialized(5),
            vec![
                5,
                ACCEL_COUNTED_EXCLUSION,
                2,
                u32::from_le_bytes([b'/', b'*', 0, 0]),
                u32::from_le_bytes([b'*', b'/', 0, 0]),
                0,
            ]
        );
        let a3 = Accel::CountedExclusion {
            opener: *b"/*-",
            closer: *b"-*/",
            n: 3,
        };
        assert_eq!(
            a3.to_serialized(6),
            vec![
                6,
                ACCEL_COUNTED_EXCLUSION,
                3,
                u32::from_le_bytes([b'/', b'*', b'-', 0]),
                u32::from_le_bytes([b'-', b'*', b'/', 0]),
                0,
            ]
        );
    }
}

#[cfg(test)]
mod accel_integration_tests {
    use crate::atn_deserializer::ATNDeserializer;

    /// End-to-end pin: real grammars produce tables whose accel section is
    /// actually populated, so the walker's `run_scan` path has per-state
    /// tags to work with.
    #[test]
    fn real_grammars_emit_accel_tags() {
        for grammar in ["CSV", "Spike", "XMLLexer"] {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(format!("tests/static_lexer/{grammar}.g4"));
            let units =
                dbt_antlr_codegen::atn_export::compile_atn_data(&path, &[]).expect("compile");
            let lexer = units
                .iter()
                .find(|u| u.lexer_atn_words.is_some())
                .expect("lexer unit");
            let atn = ATNDeserializer::new(None)
                .deserialize(&mut lexer.lexer_atn_words.as_ref().expect("lexer words").iter());
            let build = super::StaticLexerDfaBuilder::new().build(&atn);
            let mut tagged = 0;
            for m in 0..build.modes.len() {
                let Some(table) = build.table(m) else {
                    continue;
                };
                for s in 0..table.num_states() as u16 {
                    if table.accel_tag(s) != 0 {
                        tagged += 1;
                    }
                }
            }
            assert!(tagged > 0, "{grammar}: no accel tags emitted");
            eprintln!("{grammar}: {tagged} accel-tagged states");
        }
    }
}
