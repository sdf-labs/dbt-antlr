//! Compile-time static lexer DFA builder (feature `static-dfa-build`).
//!
//! The builder takes a lexer [`ATN`] and runs the subset construction that the
//! lazy [`crate::lexer_atn_simulator::LexerATNSimulator`] runs at runtime. It
//! drives the move and closure code of the simulator itself, so maximal munch,
//! rule priority, non-greedy loops, and action accumulation match the lazy
//! path by construction. The result is one static DFA table per lexer mode,
//! serialized as a `Vec<u32>` word stream. Generated code embeds the words.
//! A table walker (a later phase) tokenizes from the words without ATN
//! simulation.
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
//! # Word stream format (version 1)
//!
//! One `i32` value per `u32` word, except where noted. Sizes and indices are
//! `u32`. Symbols are `i32` values in `0..=0x10FFFF`. All multi-word sections
//! use the order of this list:
//!
//! - Preamble: `MAGIC`, `STATIC_DFA_FORMAT_VERSION`, the total word count of
//!   the stream, `num_modes`.
//! - Per mode: `present` flag (0 or 1). A mode with `present == 0` has no more
//!   words.
//! - `num_states`, `start_state`.
//! - `accept_type[num_states]`: token type per state, `-1` when the state is
//!   not an accept state.
//! - `accept_actions[num_states]`: word offset of the accept action list of
//!   the state in the action-list section, `-1` when empty.
//! - `eof[num_states]`: the EOF edge entry of each state.
//! - `low[num_states * 128]`: the edge entries of each state for the symbols
//!   `0..=127`, in symbol order per state.
//! - Hi-class section: `num_classes`, then, when `num_classes > 0`,
//!   `class_starts[num_classes]`, `num_rows`, `num_row_triples`,
//!   `row_offsets[num_rows + 1]` (in triples), `num_row_triples` times
//!   `(lo_class, hi_class, target_entry)`, and `row_of[num_states]`. Class `i`
//!   covers the symbols in `class_starts[i]..class_starts[i + 1]`, or up to
//!   `0x10FFFF` for the last class. `class_starts[0]` is 128, and the bounds
//!   are the union of the hi-edge bounds of the mode. The target of a state is
//!   constant within a class. Rows are deduplicated by content. The
//!   deserializer derives the direct symbol-to-class map from `class_starts`;
//!   the stream carries no expanded map.
//! - Action-list section: `num_action_list_ints`, then the concatenated lists.
//!   A list is `len`, then `len` times the ATN encoding `(type, data1, data2)`
//!   of one lexer action. Lists are deduplicated by content.
//! - Accel section: `num_accel_ints`, then the run-acceleration entries. An
//!   entry is `(state, ACCEL_POSITIVE, w0, w1, w2, w3)` for a 128-bit bitmap
//!   of self-looping ASCII symbols, or `(state, ACCEL_EXCLUSION, nl_in_set,
//!   n_needles, needle0, needle1, needle2)` with zero-padded needles. The
//!   bitmap never contains `'\n'`. A `\n` that loops is an exclusion needle
//!   with `nl_in_set = 1`, so the walker keeps line and column exact.
//! - Escape rows: per state, `eof_escape`, `num_ranges`, then `num_ranges`
//!   times `(lo_symbol, hi_symbol, continuation_index)`. The ranges are the
//!   sorted, disjoint symbol intervals whose edge escapes. Edges with a plain
//!   target and edges that do not exist have no range. `eof_escape` is
//!   meaningful only when the EOF edge entry is `ESCAPE`. A continuation
//!   index of `NO_CONTINUATION` marks a budget escape.
//! - Continuation pool: `num_continuations`, then per continuation `kind`,
//!   `nctx`, `nctx` times `(ctx_kind, parent_local, return_state)`, `nconfigs`,
//!   and `nconfigs` times `(state, flags, alt, ctx_local, nactions)`, with
//!   `nactions` times `(action_index, behind)` after each config.
//!
//! Edge entries are `target | ACCEPT_BIT` when the target state is an accept
//! state, `target` otherwise, `NO_EDGE` when the edge does not exist, or
//! `ESCAPE` for an escape edge. Escape edges never carry `ACCEPT_BIT`. State
//! ids stay below `0x7FFE`, so `ACCEPT_BIT | state` peaks at `0xFFFD`.
//!
//! In a continuation, `kind` is `CONTINUATION_MOVED_CONFIGS` (0) for the
//! post-move, pre-closure configs of the escape edge, or
//! `CONTINUATION_SOURCE_SET` (1) for the source configs of the state. The
//! context table is topological: local id 0 is the empty context, and entry
//! `i` has local id `i + 1` with parents that refer to smaller ids. `ctx_kind`
//! is 0 (singleton); lexer contexts are always empty or singleton chains. In
//! `flags`, bit 0 is `passed_through_non_greedy`, bit 1 is
//! `alt_reached_accept` (meaningful for kind 0 only; the resume recomputes the
//! flag for kind 1), and bit 2 is reserved. `behind` is the count of consumed
//! symbols between the action and the escape edge, or
//! `BEHIND_POSITION_INDEPENDENT` for a position-independent action. This
//! builder only emits `BEHIND_POSITION_INDEPENDENT`.
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

use std::collections::{BTreeSet, HashMap, HashSet};

pub use crate::static_lexer_dfa::{
    ACCEL_EXCLUSION, ACCEL_POSITIVE, ACCEPT_BIT, BEHIND_POSITION_INDEPENDENT,
    CONTINUATION_MOVED_CONFIGS, CONTINUATION_SOURCE_SET, ESCAPE, NO_CONTINUATION, NO_EDGE,
    STATE_MASK, STATIC_DFA_FORMAT_VERSION, STATIC_DFA_MAGIC,
};

use crate::atn::ATN;
use crate::atn_config::LexerATNConfig;
use crate::atn_config_set::MutableLexerATNConfigSet;
use crate::atn_state::ATNStateType;
use crate::int_stream::EOF;
use crate::lexer_atn_simulator::{reachable_core, start_state_core, LexerClosureEnv, ReachCapture};
use crate::prediction_context::PredictionContextRef;
use crate::transition::{Transition, TransitionType};

/// The default per-mode state budget, equal to the dbt `LexerDFABuilder`
/// default.
pub const DEFAULT_MAX_STATES_PER_MODE: usize = 30000;

const MAX_SYMBOL: i32 = 0x10FFFF;
const ASCII_MAX: i32 = 127;
const MAX_TABLE_STATE_COUNT: usize = 0x7FFE;
const MAX_CONTEXT_DEPTH: u32 = 32;
const MIN_ACCEL_LOOP_BYTES: usize = 12;

/// The result of [`StaticLexerDfaBuilder::build`]: the serialized tables and
/// structured diagnostics per mode.
#[derive(Debug)]
pub struct StaticLexerDfaBuild {
    /// The word stream, in the format of the module documentation.
    pub words: Vec<u32>,
    /// Per-mode build results, in mode order.
    pub modes: Vec<ModeBuildInfo>,
}

/// The build result of one mode.
#[derive(Debug, Clone)]
pub struct ModeBuildInfo {
    /// Whether the mode has a table in the word stream.
    pub present: bool,
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
        let mut words = vec![
            STATIC_DFA_MAGIC,
            STATIC_DFA_FORMAT_VERSION,
            0,
            num_modes as u32,
        ];
        let mut modes = Vec::with_capacity(num_modes);
        for mode in 0..num_modes {
            modes.push(self.build_mode(atn, mode, &mut words));
        }
        words[2] = words.len() as u32;
        StaticLexerDfaBuild { words, modes }
    }

    fn build_mode(&self, atn: &ATN, mode_index: usize, out: &mut Vec<u32>) -> ModeBuildInfo {
        let scratch = bumpalo::Bump::new();
        let mut mode = ModeBuild::new(self.max_states_per_mode.min(MAX_TABLE_STATE_COUNT));

        let start_state = atn.mode_to_start_state[mode_index];
        let mut env = BuildEnv::default();
        let start_set = start_state_core(atn, start_state.as_ref(), &mut env, &scratch);
        let start_configs: Vec<_> = start_set.get_items().cloned().collect();
        let start_key = key_of_configs(&mut mode.ctx_arena, atn, &start_configs);
        let failure = if env.predicate_crossed {
            Some(ModeFailure::StartClosurePredicate)
        } else if start_key.custom_action {
            Some(ModeFailure::StartClosureCustomAction)
        } else if start_key.recursive_context {
            Some(ModeFailure::StartClosureRecursiveContext)
        } else {
            None
        };
        if let Some(failure) = failure {
            out.push(0);
            return ModeBuildInfo::absent(failure);
        }
        match intern(&mut mode, atn, start_key.configs, start_configs) {
            Some(_) => {}
            None => {
                out.push(0);
                return ModeBuildInfo::absent(ModeFailure::StartStateBudgetExceeded);
            }
        }

        let mut cursor = 0;
        while cursor < mode.states.len() {
            let source = mode.states[cursor].configs.clone();
            let rows = compute_state_rows(&mut mode, atn, &source, &scratch);
            mode.rows.push(rows);
            cursor += 1;
        }

        let info = ModeBuildInfo {
            present: true,
            failure: None,
            num_states: mode.states.len(),
            num_continuations: mode.continuations.len(),
            escape_counts: mode.counts,
        };
        out.push(1);
        serialize_mode(&mode, out);
        info
    }
}

impl ModeBuildInfo {
    fn absent(failure: ModeFailure) -> Self {
        ModeBuildInfo {
            present: false,
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

/// Arena of singleton context chains for state identity. Id 0 is the empty
/// context. Entry `i` has id `i + 1`.
#[derive(Debug, Default)]
struct CtxArena {
    nodes: Vec<(u32, i32)>,
    intern: HashMap<(u32, i32), u32>,
}

impl CtxArena {
    fn push(&mut self, parent: u32, return_state: i32) -> u32 {
        if let Some(&id) = self.intern.get(&(parent, return_state)) {
            return id;
        }
        let id = (self.nodes.len() + 1) as u32;
        self.nodes.push((parent, return_state));
        self.intern.insert((parent, return_state), id);
        id
    }

    /// True when the chain has a repeated return state or is deeper than
    /// [`MAX_CONTEXT_DEPTH`].
    fn has_recursive_context(&self, mut id: u32) -> bool {
        let mut seen = HashSet::new();
        let mut depth = 0;
        while id != 0 {
            depth += 1;
            let (parent, return_state) = self.nodes[id as usize - 1];
            if depth > MAX_CONTEXT_DEPTH || !seen.insert(return_state) {
                return true;
            }
            id = parent;
        }
        false
    }
}

/// Interns the context chain of a config and returns its arena id.
fn intern_ctx(arena: &mut CtxArena, ctx: Option<PredictionContextRef>) -> u32 {
    let mut return_states = Vec::new();
    let mut cur = ctx;
    while let Some(c) = cur {
        if c.is_empty() {
            break;
        }
        assert_eq!(
            c.length(),
            1,
            "lexer prediction contexts are singleton chains"
        );
        return_states.push(c.get_return_state(0).get_state_number());
        cur = c.get_parent(0);
    }
    let mut id = 0;
    for &return_state in return_states.iter().rev() {
        id = arena.push(id, return_state);
    }
    id
}

/// The identity of one lexer config for state interning.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ConfigKey {
    state: i32,
    alt: i32,
    ctx: u32,
    non_greedy: bool,
    actions: Vec<i32>,
}

/// The identity of a config set, plus the escape signals found on the way.
#[derive(Debug)]
struct ConfigSetKey {
    configs: Vec<ConfigKey>,
    custom_action: bool,
    recursive_context: bool,
}

fn key_of_configs(
    ctx_arena: &mut CtxArena,
    atn: &ATN,
    configs: &[LexerATNConfig<'_>],
) -> ConfigSetKey {
    let mut keys = Vec::with_capacity(configs.len());
    let mut custom_action = false;
    let mut recursive_context = false;
    for config in configs {
        let ctx = intern_ctx(ctx_arena, config.get_context());
        recursive_context |= ctx_arena.has_recursive_context(ctx);
        let actions = match config_actions(atn, config) {
            Ok(actions) => actions,
            Err(()) => {
                custom_action = true;
                Vec::new()
            }
        };
        keys.push(ConfigKey {
            state: config.get_state().get_state_number(),
            alt: config.get_alt(),
            ctx,
            non_greedy: config.has_passed_through_non_greedy_decision(),
            actions,
        });
    }
    ConfigSetKey {
        configs: keys,
        custom_action,
        recursive_context,
    }
}

/// The ATN indices of the actions of the executor of a config. Fails on a
/// position-dependent action.
fn config_actions(atn: &ATN, config: &LexerATNConfig<'_>) -> Result<Vec<i32>, ()> {
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
            actions.push(index as i32);
        }
    }
    Ok(actions)
}

/// One table state: the closed config set in insertion order and the accept
/// data.
#[derive(Debug)]
struct StateData<'sc> {
    configs: Vec<LexerATNConfig<'sc>>,
    accept_type: i32,
    accept_actions: Vec<(i32, i32, i32)>,
}

/// One edge interval of a state. Covers `lo..=hi`, both inclusive.
#[derive(Debug, Clone, Copy)]
struct EdgeInterval {
    lo: i32,
    hi: i32,
    entry: u16,
    /// The continuation index when `entry == ESCAPE`.
    escape: u32,
}

/// The edges of one state: the EOF edge and the sorted, disjoint symbol
/// intervals that have an edge.
#[derive(Debug)]
struct StateRows {
    eof_entry: u16,
    /// The continuation index when `eof_entry == ESCAPE`.
    eof_escape: u32,
    intervals: Vec<EdgeInterval>,
}

/// The in-progress build of one mode.
#[derive(Debug)]
struct ModeBuild<'sc> {
    states: Vec<StateData<'sc>>,
    rows: Vec<StateRows>,
    ids: HashMap<Vec<ConfigKey>, u32>,
    ctx_arena: CtxArena,
    continuations: Vec<Vec<u32>>,
    cont_ids: HashMap<Vec<u32>, u32>,
    counts: EscapeCounts,
    max_states: usize,
}

impl<'sc> ModeBuild<'sc> {
    fn new(max_states: usize) -> Self {
        ModeBuild {
            states: Vec::new(),
            rows: Vec::new(),
            ids: HashMap::new(),
            ctx_arena: CtxArena::default(),
            continuations: Vec::new(),
            cont_ids: HashMap::new(),
            counts: EscapeCounts::default(),
            max_states,
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
}

#[derive(Debug)]
struct EdgeOutcome {
    entry: u16,
    escape: u32,
}

/// Runs move+closure for one edge of a state and classifies the result.
fn compute_edge<'sc>(
    mode: &mut ModeBuild<'sc>,
    atn: &ATN,
    source: &[LexerATNConfig<'sc>],
    t: i32,
    scratch: &'sc bumpalo::Bump,
) -> EdgeOutcome {
    let mut env = BuildEnv::default();
    let mut reach = MutableLexerATNConfigSet::new(scratch);
    let mut capture = ReachCapture::default();
    reachable_core(
        atn,
        0,
        source.iter(),
        &mut reach,
        t,
        &mut env,
        Some(&mut capture),
        scratch,
    );
    let closed: Vec<LexerATNConfig<'sc>> = reach.get_items().cloned().collect();
    let key = key_of_configs(&mut mode.ctx_arena, atn, &closed);

    let reason = if env.predicate_crossed {
        Some(EscapeReason::Predicate)
    } else if key.custom_action {
        Some(EscapeReason::CustomAction)
    } else if key.recursive_context {
        Some(EscapeReason::RecursiveContext)
    } else {
        None
    };

    if let Some(reason) = reason {
        // An empty build-time reach is a real NO_EDGE: with no pruning
        // activity, the runtime closure is a subset of the build-time closure
        // (predicates can only remove configs), so it stays empty. With
        // pruning activity, pruned configs can move at runtime: escape.
        if closed.is_empty() && !capture.pruning_activity {
            return EdgeOutcome {
                entry: NO_EDGE,
                escape: NO_CONTINUATION,
            };
        }
        mode.count_escape(reason);
        let cont = if capture.pruning_activity {
            let source_configs: Vec<(LexerATNConfig<'sc>, bool)> =
                source.iter().map(|c| (c.clone(), false)).collect();
            serialize_continuation(mode, atn, CONTINUATION_SOURCE_SET, &source_configs)
        } else {
            serialize_continuation(mode, atn, CONTINUATION_MOVED_CONFIGS, &capture.moved)
        };
        return EdgeOutcome {
            entry: ESCAPE,
            escape: cont,
        };
    }

    if closed.is_empty() {
        return EdgeOutcome {
            entry: NO_EDGE,
            escape: NO_CONTINUATION,
        };
    }
    match intern(mode, atn, key.configs, closed) {
        Some(target) => {
            let mut entry = target as u16;
            if mode.states[target as usize].accept_type >= 0 {
                entry |= ACCEPT_BIT;
            }
            EdgeOutcome {
                entry,
                escape: NO_CONTINUATION,
            }
        }
        None => {
            mode.count_escape(EscapeReason::Budget);
            EdgeOutcome {
                entry: ESCAPE,
                escape: NO_CONTINUATION,
            }
        }
    }
}

/// The accept data of a closed config set: the first rule-stop config in
/// insertion order decides, mirroring `DFA::add_lexer_state`.
fn accept_of(atn: &ATN, configs: &[LexerATNConfig<'_>]) -> (i32, Vec<(i32, i32, i32)>) {
    let Some(accept) = configs
        .iter()
        .find(|c| c.get_state().state_type() == ATNStateType::RuleStop)
    else {
        return (-1, Vec::new());
    };
    let rule = accept.get_state().get_rule_index();
    let accept_type = atn.rule_to_token_type[rule as usize];
    let actions = config_actions(atn, accept)
        .expect("interned states hold no position-dependent actions")
        .into_iter()
        .map(|index| atn.lexer_actions[index as usize].to_serialized())
        .collect();
    (accept_type, actions)
}

/// Interns a closed config set as a table state. Returns `None` when the
/// state budget is spent.
fn intern<'sc>(
    mode: &mut ModeBuild<'sc>,
    atn: &ATN,
    key: Vec<ConfigKey>,
    configs: Vec<LexerATNConfig<'sc>>,
) -> Option<u32> {
    if let Some(&id) = mode.ids.get(&key) {
        return Some(id);
    }
    if mode.states.len() >= mode.max_states {
        return None;
    }
    let id = mode.states.len() as u32;
    let (accept_type, accept_actions) = accept_of(atn, &configs);
    mode.states.push(StateData {
        configs,
        accept_type,
        accept_actions,
    });
    mode.ids.insert(key, id);
    Some(id)
}

/// Serializes one continuation and deduplicates it by content.
fn serialize_continuation(
    mode: &mut ModeBuild,
    atn: &ATN,
    kind: u32,
    configs: &[(LexerATNConfig<'_>, bool)],
) -> u32 {
    let mut ctx_entries: Vec<(u32, i32)> = Vec::new();
    let mut local_ids: HashMap<u32, u32> = HashMap::from([(0, 0)]);
    let mut config_words: Vec<u32> = Vec::new();
    for (config, alt_reached_accept) in configs.iter().map(|(c, f)| (c, *f)) {
        let global = intern_ctx(&mut mode.ctx_arena, config.get_context());
        let mut chain = Vec::new();
        let mut id = global;
        while id != 0 {
            chain.push(id);
            id = mode.ctx_arena.nodes[id as usize - 1].0;
        }
        chain.reverse();
        let mut parent_local = 0;
        for gid in chain {
            if let Some(&local) = local_ids.get(&gid) {
                parent_local = local;
                continue;
            }
            let (_, return_state) = mode.ctx_arena.nodes[gid as usize - 1];
            ctx_entries.push((parent_local, return_state));
            let local = ctx_entries.len() as u32;
            local_ids.insert(gid, local);
            parent_local = local;
        }
        let mut flags = u32::from(config.has_passed_through_non_greedy_decision());
        if alt_reached_accept {
            flags |= 0b10;
        }
        let actions = config_actions(atn, config)
            .expect("escape continuations hold no position-dependent actions");
        config_words.push(config.get_state().get_state_number() as u32);
        config_words.push(flags);
        config_words.push(config.get_alt() as u32);
        config_words.push(parent_local);
        config_words.push(actions.len() as u32);
        for action in actions {
            config_words.push(action as u32);
            config_words.push(BEHIND_POSITION_INDEPENDENT);
        }
    }
    let mut body = vec![kind, ctx_entries.len() as u32];
    for &(parent, return_state) in &ctx_entries {
        body.extend([0, parent, return_state as u32]);
    }
    body.push(configs.len() as u32);
    body.extend(config_words);
    if let Some(&id) = mode.cont_ids.get(&body) {
        return id;
    }
    let id = mode.continuations.len() as u32;
    mode.cont_ids.insert(body.clone(), id);
    mode.continuations.push(body);
    id
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
    source: &[LexerATNConfig<'sc>],
    scratch: &'sc bumpalo::Bump,
) -> StateRows {
    let eof = compute_edge(mode, atn, source, EOF, scratch);
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
            let edge = compute_edge(mode, atn, source, lo, scratch);
            if edge.entry == NO_EDGE {
                lo = b;
                continue;
            }
            if let Some(last) = intervals.last_mut() {
                if last.entry == edge.entry && last.escape == edge.escape && last.hi + 1 == lo {
                    last.hi = hi;
                    lo = b;
                    continue;
                }
            }
            intervals.push(EdgeInterval {
                lo,
                hi,
                entry: edge.entry,
                escape: edge.escape,
            });
        }
        lo = b;
    }
    StateRows {
        eof_entry: eof.entry,
        eof_escape: eof.escape,
        intervals,
    }
}

/// The dense row of a state for the symbols `0..=127`.
fn low_row_of(rows: &StateRows) -> [u16; 128] {
    let mut row = [NO_EDGE; 128];
    for iv in &rows.intervals {
        if iv.lo <= ASCII_MAX {
            let hi = iv.hi.min(ASCII_MAX);
            for c in iv.lo..=hi {
                row[c as usize] = iv.entry;
            }
        }
    }
    row
}

/// The edges of a state for the symbols above 127, clipped to start at 128.
fn hi_clipped(rows: &StateRows) -> Vec<(i32, i32, u16)> {
    rows.intervals
        .iter()
        .filter(|iv| iv.hi > ASCII_MAX)
        .map(|iv| (iv.lo.max(ASCII_MAX + 1), iv.hi, iv.entry))
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

/// The serialized accel entry of one state, or empty. A port of the dbt
/// `LexerDFABuilder#computeAccel` classification.
fn accel_words(s: usize, low: &[u16; 128], hi: &[(i32, i32, u16)]) -> Vec<u32> {
    let mut bitmap = 0u128;
    let mut count = 0usize;
    for (c, &entry) in low.iter().enumerate() {
        if entry != NO_EDGE && entry != ESCAPE && (entry & STATE_MASK) as usize == s {
            bitmap |= 1u128 << c;
            count += 1;
        }
    }
    if count == 0 {
        return Vec::new();
    }
    let nl_loops = bitmap & (1 << b'\n') != 0;
    let all_hi_loop = hi.len() == 1
        && hi[0].0 == ASCII_MAX + 1
        && hi[0].1 == MAX_SYMBOL
        && hi[0].2 != NO_EDGE
        && hi[0].2 != ESCAPE
        && (hi[0].2 & STATE_MASK) as usize == s;
    if all_hi_loop {
        let mut needles: Vec<u32> = (0..128u32).filter(|&c| bitmap & (1 << c) == 0).collect();
        if nl_loops {
            needles.push(u32::from(b'\n'));
        }
        if (1..=3).contains(&needles.len()) {
            let n = needles.len() as u32;
            needles.resize(3, 0);
            let mut words = vec![s as u32, ACCEL_EXCLUSION, u32::from(nl_loops), n];
            words.extend(needles);
            return words;
        }
    }
    if nl_loops {
        bitmap &= !(1u128 << b'\n');
    }
    if count - usize::from(nl_loops) < MIN_ACCEL_LOOP_BYTES {
        return Vec::new();
    }
    vec![
        s as u32,
        ACCEL_POSITIVE,
        bitmap as u32,
        (bitmap >> 32) as u32,
        (bitmap >> 64) as u32,
        (bitmap >> 96) as u32,
    ]
}

/// Writes one present mode to the word stream, in the section order of the
/// module documentation.
fn serialize_mode(mode: &ModeBuild, out: &mut Vec<u32>) {
    let num_states = mode.states.len();
    out.push(num_states as u32);
    out.push(0); // start_state: the start state is interned first.

    let mut action_lists: Vec<u32> = Vec::new();
    let mut action_list_ids: HashMap<&[(i32, i32, i32)], u32> = HashMap::new();
    let mut accept_action_offsets = Vec::with_capacity(num_states);
    for state in &mode.states {
        if state.accept_actions.is_empty() {
            accept_action_offsets.push(-1);
            continue;
        }
        let offset = *action_list_ids
            .entry(&state.accept_actions)
            .or_insert_with(|| {
                let offset = action_lists.len() as u32;
                action_lists.push(state.accept_actions.len() as u32);
                for &(ty, d1, d2) in &state.accept_actions {
                    action_lists.extend([ty as u32, d1 as u32, d2 as u32]);
                }
                offset
            });
        accept_action_offsets.push(offset as i32);
    }
    for state in &mode.states {
        out.push(state.accept_type as u32);
    }
    for offset in accept_action_offsets {
        out.push(offset as u32);
    }

    let low_rows: Vec<[u16; 128]> = mode.rows.iter().map(low_row_of).collect();
    for rows in &mode.rows {
        out.push(u32::from(rows.eof_entry));
    }
    for low in &low_rows {
        out.extend(low.iter().map(|&entry| u32::from(entry)));
    }

    serialize_hi_classes(mode, out);

    out.push(action_lists.len() as u32);
    out.extend(action_lists);

    let mut accel = Vec::new();
    for (s, (rows, low)) in mode.rows.iter().zip(&low_rows).enumerate() {
        accel.extend(accel_words(s, low, &hi_clipped(rows)));
    }
    out.push(accel.len() as u32);
    out.extend(accel);

    for rows in &mode.rows {
        out.push(rows.eof_escape);
        let escapes = rows.intervals.iter().filter(|iv| iv.entry == ESCAPE);
        out.push(escapes.clone().count() as u32);
        for iv in escapes {
            out.push(iv.lo as u32);
            out.push(iv.hi as u32);
            out.push(iv.escape);
        }
    }

    out.push(mode.continuations.len() as u32);
    for body in &mode.continuations {
        out.extend_from_slice(body);
    }
}

/// Writes the class-compressed hi-edge section of one mode.
fn serialize_hi_classes(mode: &ModeBuild, out: &mut Vec<u32>) {
    let class_starts = collect_class_starts(&mode.rows);
    out.push(class_starts.len() as u32);
    if class_starts.is_empty() {
        return;
    }
    out.extend(class_starts.iter().map(|&s| s as u32));

    let mut rows: Vec<Vec<(u32, u32, u16)>> = Vec::new();
    let mut row_ids: HashMap<Vec<(u32, u32, u16)>, u32> = HashMap::new();
    let mut row_of = Vec::with_capacity(mode.rows.len());
    for state_rows in &mode.rows {
        let runs = class_runs(&class_starts, &hi_clipped(state_rows));
        let id = *row_ids.entry(runs).or_insert_with_key(|runs| {
            rows.push(runs.clone());
            (rows.len() - 1) as u32
        });
        row_of.push(id);
    }

    out.push(rows.len() as u32);
    out.push(rows.iter().map(Vec::len).sum::<usize>() as u32);
    let mut offset = 0;
    let mut row_offsets = Vec::with_capacity(rows.len() + 1);
    for row in &rows {
        row_offsets.push(offset);
        offset += row.len() as u32;
    }
    row_offsets.push(offset);
    out.extend(row_offsets);
    for row in &rows {
        for &(lo_class, hi_class, entry) in row {
            out.extend([lo_class, hi_class, u32::from(entry)]);
        }
    }
    out.extend(row_of);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ctx_arena_detects_recursion() {
        let mut arena = CtxArena::default();
        let a = arena.push(0, 10);
        let b = arena.push(a, 20);
        assert!(!arena.has_recursive_context(b));
        let c = arena.push(b, 10);
        assert!(arena.has_recursive_context(c));
        let mut deep = 0;
        for i in 0..33 {
            deep = arena.push(deep, 100 + i);
        }
        assert!(arena.has_recursive_context(deep));
    }

    #[test]
    fn class_starts_cover_hi_edge_bounds() {
        let rows = vec![StateRows {
            eof_entry: NO_EDGE,
            eof_escape: NO_CONTINUATION,
            intervals: vec![
                EdgeInterval {
                    lo: 65,
                    hi: 200,
                    entry: 3,
                    escape: NO_CONTINUATION,
                },
                EdgeInterval {
                    lo: 0x1000,
                    hi: 0x2000,
                    entry: 4,
                    escape: NO_CONTINUATION,
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
            eof_entry: NO_EDGE,
            eof_escape: NO_CONTINUATION,
            intervals: vec![EdgeInterval {
                lo: 65,
                hi: 90,
                entry: 3,
                escape: NO_CONTINUATION,
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

    #[test]
    fn accel_exclusion_with_newline_needle() {
        // State 2 loops on every symbol except 'a', 'b', and '\n' loops too.
        let mut low = [NO_EDGE; 128];
        for entry in low.iter_mut() {
            *entry = 2;
        }
        low[b'a' as usize] = NO_EDGE;
        low[b'b' as usize] = NO_EDGE;
        let hi = vec![(128, MAX_SYMBOL, 2u16)];
        let words = accel_words(2, &low, &hi);
        assert_eq!(
            words,
            vec![
                2,
                ACCEL_EXCLUSION,
                1,
                3,
                u32::from(b'a'),
                u32::from(b'b'),
                u32::from(b'\n')
            ]
        );
    }

    #[test]
    fn accel_positive_excludes_newline() {
        // State 1 loops on 13 ASCII symbols including '\n'; no hi edges.
        let mut low = [NO_EDGE; 128];
        for c in 0..13u8 {
            low[c as usize] = 1;
        }
        let words = accel_words(1, &low, &[]);
        assert_eq!(words.len(), 6);
        assert_eq!(words[0], 1);
        assert_eq!(words[1], ACCEL_POSITIVE);
        let mut bitmap = 0u128;
        for i in 0..4 {
            bitmap |= u128::from(words[2 + i]) << (i * 32);
        }
        assert_eq!(bitmap & (1 << b'\n'), 0);
        assert_eq!(bitmap.count_ones(), 12);
    }

    #[test]
    fn accel_none_for_short_loops() {
        let mut low = [NO_EDGE; 128];
        for c in b'0'..=b'9' {
            low[c as usize] = 3;
        }
        assert!(accel_words(3, &low, &[]).is_empty());
    }
}

#[cfg(test)]
mod accel_integration_tests {
    use crate::atn_deserializer::ATNDeserializer;
    use crate::static_lexer_dfa::StaticLexerTables;

    /// End-to-end pin: real grammars produce tables whose accel section is
    /// actually populated, and the loader decodes nonzero per-state tags
    /// (the walker's `run_scan` path depends on this).
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
            let tables = StaticLexerTables::from_words(&build.words).expect("decode");
            let mut tagged = 0;
            for m in 0..tables.num_modes() {
                let Some(table) = tables.mode(m) else {
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
