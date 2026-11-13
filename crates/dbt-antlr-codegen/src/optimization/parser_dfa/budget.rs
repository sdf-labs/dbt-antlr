// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The analysis budgets of the static parser DFA construction.
//!
//! Each decision gets a [`DecisionBudget`]: the state cap of its table, the
//! lookahead depth of a hybrid table, and the configs that one build attempt
//! can visit. The limits come from these sources, from the highest priority
//! to the lowest:
//!
//! 1. A `--dfa-budget` selector that matches the decision.
//! 2. The options of the block that owns the decision.
//! 3. The options of the rule of the decision.
//! 4. The options of the grammar.
//! 5. The global settings (`--max-dfa-states`, `--max-dfa-depth`,
//!    `--max-dfa-visits`).
//! 6. The defaults.
//!
//! Each limit resolves on its own, so a rule can set `max_k` and keep the
//! state cap of the grammar.

use dbt_antlr_runtime::dfa::static_parser_dfa::{MAX_TABLE_CELLS, MAX_TABLE_STATES};

use crate::grammar::model::OptionDecl;
pub(crate) use crate::optimization::options::BudgetLimits;

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

/// The decisions that one `--dfa-budget` setting applies to.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum BudgetSelector {
    Decision(usize),
    Rule(String),
    RuleDecision { rule: String, decision: usize },
}

impl BudgetSelector {
    fn parse(parts: &[&str]) -> Option<Self> {
        match parts {
            [item] => Some(match decision_number(item) {
                Some(decision) => Self::Decision(decision),
                None if is_rule_name(item) => Self::Rule((*item).to_owned()),
                None => return None,
            }),
            [rule, decision] if is_rule_name(rule) => Some(Self::RuleDecision {
                rule: (*rule).to_owned(),
                decision: decision_number(decision)?,
            }),
            _ => None,
        }
    }

    /// Whether the selector matches decision `decision` of rule `rule`.
    pub(crate) fn matches(&self, rule: &str, decision: usize) -> bool {
        match self {
            Self::Decision(wanted) => *wanted == decision,
            Self::Rule(wanted) => wanted == rule,
            Self::RuleDecision {
                rule: wanted_rule,
                decision: wanted,
            } => wanted_rule == rule && *wanted == decision,
        }
    }
}

fn decision_number(text: &str) -> Option<usize> {
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

fn is_rule_name(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|character| character.is_alphanumeric() || character == '_')
}

/// One `--dfa-budget` setting: `SELECTOR:SETTING[,SETTING...]`, where the
/// selector is a decision number, a rule name, or `rule:decision`, and each
/// setting is `states=N`, `k=N`, or `visits=N`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BudgetRule {
    pub(crate) selector: BudgetSelector,
    pub(crate) limits: BudgetLimits,
    /// The selector as the user wrote it, for diagnostics.
    pub(crate) text: String,
}

impl BudgetRule {
    pub(crate) fn parse(spec: &str) -> Result<Self, String> {
        let malformed = |reason: &str| {
            format!(
                "invalid --dfa-budget '{spec}': {reason}; expected \
                 SELECTOR:SETTING[,SETTING...] with a decision number, a rule name, or \
                 rule:decision as the selector, and states=N, k=N, or visits=N as settings"
            )
        };
        let parts = spec.split(':').map(str::trim).collect::<Vec<_>>();
        let Some((settings, selector)) = parts.split_last() else {
            return Err(malformed("the setting is missing"));
        };
        let selector = BudgetSelector::parse(selector)
            .ok_or_else(|| malformed("the selector is not valid"))?;
        let mut limits = BudgetLimits::default();
        for setting in settings.split(',').map(str::trim) {
            let Some((name, value)) = setting.split_once('=') else {
                return Err(malformed(&format!("'{setting}' is not NAME=VALUE")));
            };
            let (name, value) = (name.trim(), value.trim());
            if !matches!(name, "states" | "k" | "visits") {
                return Err(malformed(&format!("'{name}' is not a known setting")));
            }
            let option = match name {
                "states" => MAX_STATES_OPTION,
                "k" => MAX_K_OPTION,
                _ => MAX_VISITS_OPTION,
            };
            if let Some(problem) = budget_option_problem(option, value) {
                return Err(malformed(&format!("{name}: {problem}")));
            }
            limits.set(option, value);
        }
        let text = parts[..parts.len() - 1].join(":");
        Ok(Self {
            selector,
            limits,
            text,
        })
    }
}

/// The budget settings of one generation run: the global limits and the
/// `--dfa-budget` settings, in command-line order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ParserDfaBudgets {
    pub(crate) global: BudgetLimits,
    pub(crate) rules: Vec<BudgetRule>,
}

impl ParserDfaBudgets {
    /// The limits of the `--dfa-budget` settings that match decision
    /// `decision` of rule `rule`. A later setting wins over an earlier one.
    pub(crate) fn selected_limits(&self, rule: &str, decision: usize) -> BudgetLimits {
        self.rules
            .iter()
            .rev()
            .filter(|budget| budget.selector.matches(rule, decision))
            .fold(BudgetLimits::default(), |limits, budget| {
                limits.or(budget.limits)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn budget_rules_parse_each_selector_form() {
        let rule = BudgetRule::parse("selectStmt:states=4000,k=6").expect("valid");
        assert_eq!(rule.selector, BudgetSelector::Rule("selectStmt".to_owned()));
        assert_eq!(rule.text, "selectStmt");
        assert_eq!(
            rule.limits,
            BudgetLimits {
                max_states: Some(4000),
                max_k: Some(6),
                max_visits: None,
            }
        );

        let rule = BudgetRule::parse("12:visits=5000000").expect("valid");
        assert_eq!(rule.selector, BudgetSelector::Decision(12));
        assert_eq!(rule.limits.max_visits, Some(5_000_000));

        let rule = BudgetRule::parse("expr:251:states=3000").expect("valid");
        assert_eq!(
            rule.selector,
            BudgetSelector::RuleDecision {
                rule: "expr".to_owned(),
                decision: 251,
            }
        );
        assert_eq!(rule.text, "expr:251");
    }

    #[test]
    fn budget_rules_reject_malformed_specs() {
        for spec in [
            "states=4000",
            "expr:",
            "expr:depth=4",
            "expr:states=0",
            "expr:states=40000",
            "expr:k=x",
            "a:b:c:k=2",
            "-:k=2",
        ] {
            assert!(
                BudgetRule::parse(spec).is_err(),
                "{spec} should be rejected"
            );
        }
    }

    #[test]
    fn later_budget_rules_win_per_limit() {
        let budgets = ParserDfaBudgets {
            global: BudgetLimits::default(),
            rules: vec![
                BudgetRule::parse("expr:states=2000,k=5").expect("valid"),
                BudgetRule::parse("expr:3:k=7").expect("valid"),
                BudgetRule::parse("other:states=9").expect("valid"),
            ],
        };
        assert_eq!(
            budgets.selected_limits("expr", 3),
            BudgetLimits {
                max_states: Some(2000),
                max_k: Some(7),
                max_visits: None,
            }
        );
        assert_eq!(budgets.selected_limits("expr", 4).max_k, Some(5));
        assert_eq!(budgets.selected_limits("stmt", 3), BudgetLimits::default());
    }

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
