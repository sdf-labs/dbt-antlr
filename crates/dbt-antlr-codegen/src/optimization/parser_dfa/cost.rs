// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! What the classification of one decision cost: the build attempts, the
//! configs their closures visited, and the rules those configs belong to.
//!
//! The counts are deterministic, so the `G4S193` warnings that use them are
//! the same on every machine. Wall-clock times are only measured for
//! `--profile-decisions`.

use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AttemptKind {
    /// A static attempt with no depth bound. `abort` is `true` when the
    /// attempt stops at an untrusted conflict.
    Unbounded { abort: bool },
    /// A static attempt with full widening.
    Widened { abort: bool },
    /// A hybrid attempt with the default budget.
    Hybrid,
    /// A hybrid attempt with the larger `--static-dfa-only` budget.
    StaticOnlyHybrid,
}

impl AttemptKind {
    /// The code of the attempt in the `--profile-decisions` report: `U`, `W`,
    /// `H`, or `S`. A static attempt of the witness pass, which does not stop
    /// at an untrusted conflict, has a lowercase code.
    fn code(self) -> char {
        match self {
            Self::Unbounded { abort: true } => 'U',
            Self::Unbounded { abort: false } => 'u',
            Self::Widened { abort: true } => 'W',
            Self::Widened { abort: false } => 'w',
            Self::Hybrid => 'H',
            Self::StaticOnlyHybrid => 'S',
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AttemptOutcome {
    /// The construction ran to its end.
    Finished,
    /// The construction stopped at the first conflict it does not trust.
    Stopped,
    /// The construction spent the DFA state budget.
    StateBudget,
    /// The closures of the construction spent the visit budget.
    VisitBudget,
}

impl AttemptOutcome {
    /// The suffix of the attempt code in the `--profile-decisions` report.
    const fn marker(self) -> &'static str {
        match self {
            Self::Finished => "",
            Self::Stopped => "!",
            Self::StateBudget => "^",
            Self::VisitBudget => "$",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AttemptCost {
    pub(crate) kind: AttemptKind,
    /// The precedence class of the attempt, for a precedence-loop decision.
    pub(crate) class: Option<usize>,
    pub(crate) outcome: AttemptOutcome,
    /// Configs the closures of the attempt visited.
    pub(crate) visits: u64,
    /// DFA states the attempt built.
    pub(crate) states: usize,
    /// Wall-clock time of the attempt, measured only for
    /// `--profile-decisions`.
    pub(crate) elapsed: Option<Duration>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DecisionCost {
    /// The build attempts, in the order they ran.
    pub(crate) attempts: Vec<AttemptCost>,
    /// The visited configs of all attempts by the rule of their ATN state, as
    /// `(rule index, visits)`. The most visited rule comes first, and rules
    /// with no visit are left out.
    pub(crate) rule_visits: Vec<(usize, u64)>,
    /// Wall-clock time of the classification, measured only for
    /// `--profile-decisions`.
    pub(crate) elapsed: Option<Duration>,
}

impl DecisionCost {
    /// Configs visited by all attempts.
    pub(crate) fn visits(&self) -> u64 {
        self.attempts.iter().map(|attempt| attempt.visits).sum()
    }

    /// DFA states built by all attempts.
    pub(crate) fn states(&self) -> usize {
        self.attempts.iter().map(|attempt| attempt.states).sum()
    }

    /// The number of attempts that ended with `outcome`.
    pub(crate) fn count(&self, outcome: AttemptOutcome) -> usize {
        self.attempts
            .iter()
            .filter(|attempt| attempt.outcome == outcome)
            .count()
    }

    /// The attempt sequence as codes, for example `U! H u`. The attempts of a
    /// precedence class follow the class number: `c0: U; c1: U$ W`. When the
    /// attempts are timed, an attempt of 1 ms or more ends with its
    /// milliseconds: `U!(12)`.
    pub(crate) fn attempts_text(&self) -> String {
        let mut text = String::new();
        let mut class = None;
        for attempt in &self.attempts {
            if attempt.class != class || text.is_empty() {
                if !text.is_empty() {
                    text.push_str("; ");
                }
                if let Some(index) = attempt.class {
                    text.push_str(&format!("c{index}: "));
                }
                class = attempt.class;
            } else {
                text.push(' ');
            }
            text.push(attempt.kind.code());
            text.push_str(attempt.outcome.marker());
            let millis = attempt.elapsed.map_or(0, |elapsed| elapsed.as_millis());
            if millis > 0 {
                text.push_str(&format!("({millis})"));
            }
        }
        text
    }

    /// The `limit` most visited rules with their share of the visits, for
    /// example `identifier 61%, expr 18%`.
    pub(crate) fn hot_rules_text(&self, rule_names: &[String], limit: usize) -> String {
        let total = self.visits();
        if total == 0 {
            return String::new();
        }
        self.rule_visits
            .iter()
            .take(limit)
            .map(|&(rule, visits)| {
                let name = rule_names.get(rule).map_or("?", String::as_str);
                let percent = visits.saturating_mul(100) / total;
                if percent == 0 {
                    format!("{name} <1%")
                } else {
                    format!("{name} {percent}%")
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Collects the cost of one decision while the classifier runs it.
#[derive(Debug, Default)]
pub(super) struct CostLog {
    attempts: Vec<AttemptCost>,
    rule_visits: Vec<u64>,
    /// The precedence class that the next attempts build.
    pub(super) class: Option<usize>,
}

impl CostLog {
    /// Records one attempt with its visits by rule index.
    pub(super) fn record(&mut self, attempt: AttemptCost, rule_visits: &[u64]) {
        if self.rule_visits.len() < rule_visits.len() {
            self.rule_visits.resize(rule_visits.len(), 0);
        }
        for (total, visits) in self.rule_visits.iter_mut().zip(rule_visits) {
            *total += visits;
        }
        self.attempts.push(attempt);
    }

    pub(super) fn finish(self, elapsed: Option<Duration>) -> DecisionCost {
        let mut rule_visits: Vec<(usize, u64)> = self
            .rule_visits
            .into_iter()
            .enumerate()
            .filter(|&(_, visits)| visits > 0)
            .collect();
        rule_visits.sort_by(|left, right| right.1.cmp(&left.1).then(left.0.cmp(&right.0)));
        DecisionCost {
            attempts: self.attempts,
            rule_visits,
            elapsed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attempt(kind: AttemptKind, class: Option<usize>, outcome: AttemptOutcome) -> AttemptCost {
        AttemptCost {
            kind,
            class,
            outcome,
            visits: 10,
            states: 1,
            elapsed: None,
        }
    }

    #[test]
    fn attempts_text_groups_the_precedence_classes() {
        let cost = DecisionCost {
            attempts: vec![
                attempt(
                    AttemptKind::Unbounded { abort: true },
                    None,
                    AttemptOutcome::Stopped,
                ),
                attempt(AttemptKind::Hybrid, None, AttemptOutcome::Finished),
                attempt(
                    AttemptKind::Unbounded { abort: false },
                    None,
                    AttemptOutcome::VisitBudget,
                ),
            ],
            ..DecisionCost::default()
        };
        assert_eq!(cost.attempts_text(), "U! H u$");

        let mut cost = DecisionCost {
            attempts: vec![
                attempt(
                    AttemptKind::Unbounded { abort: true },
                    Some(0),
                    AttemptOutcome::Finished,
                ),
                attempt(
                    AttemptKind::Unbounded { abort: true },
                    Some(1),
                    AttemptOutcome::StateBudget,
                ),
                attempt(
                    AttemptKind::Widened { abort: true },
                    Some(1),
                    AttemptOutcome::Finished,
                ),
                attempt(
                    AttemptKind::StaticOnlyHybrid,
                    Some(1),
                    AttemptOutcome::Finished,
                ),
            ],
            ..DecisionCost::default()
        };
        assert_eq!(cost.attempts_text(), "c0: U; c1: U^ W S");
        cost.attempts[1].elapsed = Some(Duration::from_micros(12_700));
        cost.attempts[2].elapsed = Some(Duration::from_micros(300));
        assert_eq!(cost.attempts_text(), "c0: U; c1: U^(12) W S");
        assert_eq!(cost.visits(), 40);
        assert_eq!(cost.states(), 4);
        assert_eq!(cost.count(AttemptOutcome::StateBudget), 1);
    }

    #[test]
    fn rule_visits_put_the_most_visited_rule_first() {
        let mut log = CostLog::default();
        log.record(
            attempt(AttemptKind::Hybrid, None, AttemptOutcome::Finished),
            &[1, 0, 600, 399],
        );
        log.record(
            attempt(AttemptKind::Hybrid, None, AttemptOutcome::Finished),
            &[3, 0, 0, 0],
        );
        let mut cost = log.finish(None);
        assert_eq!(cost.rule_visits, [(2, 600), (3, 399), (0, 4)]);
        for attempt in &mut cost.attempts {
            attempt.visits = 500;
        }
        let names = ["a", "b", "c", "d"].map(str::to_owned);
        assert_eq!(cost.hot_rules_text(&names, 2), "c 60%, d 39%");
        assert_eq!(cost.hot_rules_text(&names, 3), "c 60%, d 39%, a <1%");
    }
}
