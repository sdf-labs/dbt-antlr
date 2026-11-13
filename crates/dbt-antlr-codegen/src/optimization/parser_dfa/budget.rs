// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The analysis budgets of the static parser DFA construction.
//!
//! Each decision gets a [`DecisionBudget`]: the state cap of its table, the
//! lookahead depth of a hybrid table, and the configs that one build attempt
//! can visit. The limits come from these sources, from the highest priority
//! to the lowest:
//!
//! 1. The options of the block that owns the decision.
//! 2. The options of the rule of the decision.
//! 3. The options of the grammar.
//! 4. The defaults.
//!
//! Each limit resolves on its own, so a rule can set `max_k` and keep the
//! state cap of the grammar.
//!
//! The budget can change the language of a `--static-dfa-only` parser,
//! because a smaller budget makes more guesses. Thus only the grammar sets
//! the budget, and the generator has no command-line setting for it.

use dbt_antlr_runtime::dfa::static_parser_dfa::{MAX_TABLE_CELLS, MAX_TABLE_STATES};

use crate::grammar::model::OptionDecl;

/// The default state cap of one decision table.
pub(crate) const DEFAULT_MAX_STATES: usize = 1000;

/// The default number of configs that one build attempt can visit.
///
/// The state cap alone does not limit the analysis time. The build-time
/// closure explores every path, and the runtime explores a path only when the
/// input needs it. Thus one state of a large grammar can close over tens of
/// thousands of configs. When a static attempt spends this budget, the
/// classifier tries a wider widening or a hybrid table. When a hybrid attempt
/// spends it after the start state, the states that are not expanded escape.
pub(crate) const DEFAULT_VISIT_BUDGET: u64 = 2_000_000;

/// The visits that one state of the state cap adds to the visit budget when
/// the visit budget is not set. With the default state cap, this gives the
/// default visit budget.
const VISITS_PER_STATE: u64 = DEFAULT_VISIT_BUDGET / DEFAULT_MAX_STATES as u64;

/// The grammar option of the state cap.
pub(crate) const MAX_STATES_OPTION: &str = "max_dfa_states";
/// The short name of [`MAX_STATES_OPTION`].
pub(crate) const MAX_STATES_ALIAS: &str = "max_states";
/// The grammar option of the hybrid lookahead depth.
pub(crate) const MAX_K_OPTION: &str = "max_k";
/// The grammar option of the visit budget.
pub(crate) const MAX_VISITS_OPTION: &str = "max_dfa_visits";

/// The grammar, rule, and block options that set a budget.
pub(crate) const BUDGET_OPTIONS: &[&str] = &[
    MAX_STATES_OPTION,
    MAX_STATES_ALIAS,
    MAX_K_OPTION,
    MAX_VISITS_OPTION,
];

/// Whether `name` is a budget option.
pub(crate) fn is_budget_option(name: &str) -> bool {
    BUDGET_OPTIONS.contains(&name)
}

/// Whether one of `options` is not a budget option. Budget options change
/// only the analysis, so a transform can ignore them.
pub(crate) fn has_non_budget_option(options: &[OptionDecl]) -> bool {
    options
        .iter()
        .any(|option| !is_budget_option(&option.name.value))
}

/// The problem with the value of a budget option, or `None` when the value is
/// correct or the option is not a budget option.
pub(crate) fn budget_option_problem(name: &str, value: &str) -> Option<String> {
    match name {
        MAX_STATES_OPTION | MAX_STATES_ALIAS => parse_limit(value, MAX_TABLE_STATES as u64).err(),
        MAX_K_OPTION => parse_limit(value, u64::from(u32::MAX)).err(),
        MAX_VISITS_OPTION => parse_limit(value, u64::MAX).err(),
        _ => None,
    }
}

/// A positive integer of at most `max`.
fn parse_limit(value: &str, max: u64) -> Result<u64, String> {
    let parsed = value
        .parse::<u64>()
        .map_err(|_| format!("'{value}' is not a positive integer"))?;
    if parsed == 0 {
        return Err("the value must be at least 1".to_owned());
    }
    if parsed > max {
        return Err(format!("the value must be at most {max}"));
    }
    Ok(parsed)
}

/// The analysis budget limits that one source sets. A limit that is `None`
/// comes from a source with a lower priority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct BudgetLimits {
    /// The state cap of a decision table.
    pub(crate) max_states: Option<usize>,
    /// The depth at which a hybrid table stops.
    pub(crate) max_k: Option<usize>,
    /// The configs that one build attempt can visit.
    pub(crate) max_visits: Option<u64>,
}

impl BudgetLimits {
    /// Each limit of `self`, or the limit of `fallback` when `self` does not
    /// set it.
    pub(crate) const fn or(self, fallback: Self) -> Self {
        Self {
            max_states: match self.max_states {
                Some(value) => Some(value),
                None => fallback.max_states,
            },
            max_k: match self.max_k {
                Some(value) => Some(value),
                None => fallback.max_k,
            },
            max_visits: match self.max_visits {
                Some(value) => Some(value),
                None => fallback.max_visits,
            },
        }
    }

    /// The limits that the budget options of `options` set. The semantic
    /// checks report an incorrect value, so this ignores it.
    pub(crate) fn from_options(options: &[OptionDecl]) -> Self {
        let mut limits = Self::default();
        for option in options {
            limits.set(&option.name.value, &option.value.value);
        }
        limits
    }

    /// Sets the limit of the budget option `name` to `value`. Does nothing
    /// when `name` is not a budget option or `value` is not correct.
    fn set(&mut self, name: &str, value: &str) {
        if !is_budget_option(name) || budget_option_problem(name, value).is_some() {
            return;
        }
        let Ok(parsed) = value.parse::<u64>() else {
            return;
        };
        match name {
            MAX_STATES_OPTION | MAX_STATES_ALIAS => self.max_states = usize::try_from(parsed).ok(),
            MAX_K_OPTION => self.max_k = usize::try_from(parsed).ok(),
            _ => self.max_visits = Some(parsed),
        }
    }
}

/// The resolved budget of one decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct DecisionBudget {
    /// The state cap of one table of the decision.
    pub(crate) max_states: usize,
    /// The depth at which a hybrid table stops, or `None` for the
    /// default depth of the attempt.
    pub(crate) max_k: Option<usize>,
    /// The configs that one build attempt can visit.
    pub(crate) max_visits: u64,
    /// A source set the state cap. The default hybrid table then expands up
    /// to the state cap too, and not only up to its own small cap.
    pub(crate) explicit_max_states: bool,
}

impl Default for DecisionBudget {
    fn default() -> Self {
        Self {
            max_states: DEFAULT_MAX_STATES,
            max_k: None,
            max_visits: DEFAULT_VISIT_BUDGET,
            explicit_max_states: false,
        }
    }
}

impl DecisionBudget {
    /// The budget that `limits` give for a parser with tokens up to
    /// `max_token_type`.
    ///
    /// This function clamps the state cap to what the table image can hold: at
    /// most [`MAX_TABLE_STATES`] states, and at most [`MAX_TABLE_CELLS`] row
    /// cells when each state has a row of its own. When `limits` does not set
    /// the visit budget, the visit budget grows with the state cap.
    pub(crate) fn resolve(limits: BudgetLimits, max_token_type: i32) -> Self {
        let row_width = usize::try_from(max_token_type).unwrap_or(0) + 2;
        let image_cap = MAX_TABLE_STATES.min(MAX_TABLE_CELLS / row_width).max(1);
        let max_states = limits
            .max_states
            .unwrap_or(DEFAULT_MAX_STATES)
            .clamp(1, image_cap);
        let max_visits = limits
            .max_visits
            .unwrap_or_else(|| DEFAULT_VISIT_BUDGET.max(max_states as u64 * VISITS_PER_STATE));
        Self {
            max_states,
            max_k: limits.max_k,
            max_visits,
            explicit_max_states: limits.max_states.is_some(),
        }
    }
}

/// The base limits of the static parser DFA pass, under the options of the
/// grammar. Code generation always uses the defaults. Tests lower the limits
/// to examine the behavior at the edge of a budget.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ParserDfaBudgets {
    pub(crate) global: BudgetLimits,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolved_budgets_clamp_to_the_table_image() {
        let budget = DecisionBudget::resolve(BudgetLimits::default(), 100);
        assert_eq!(budget, DecisionBudget::default());

        let budget = DecisionBudget::resolve(
            BudgetLimits {
                max_states: Some(4000),
                ..BudgetLimits::default()
            },
            100,
        );
        assert_eq!(budget.max_states, 4000);
        assert_eq!(budget.max_visits, 8_000_000);

        let budget = DecisionBudget::resolve(
            BudgetLimits {
                max_states: Some(MAX_TABLE_STATES),
                max_visits: Some(10),
                ..BudgetLimits::default()
            },
            2000,
        );
        assert_eq!(budget.max_states, MAX_TABLE_CELLS / 2002);
        assert_eq!(budget.max_visits, 10);
    }
}
