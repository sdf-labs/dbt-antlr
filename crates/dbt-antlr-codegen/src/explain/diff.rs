// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The comparison of the static parser trace with the full `LL` trace.
//!
//! Both simulations visit the same decisions in the same order until one of
//! them chooses another alternative. The first such decision is the root
//! cause of every later difference. Full `LL` prediction is exact: when it
//! fails, the grammar itself rejects the input.

use super::interp::{Outcome, Resolution, Trace};

/// What the comparison found.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Verdict {
    /// Both parsers made the same choices.
    Agree,
    /// The static parser chose another alternative than full `LL` at `step`.
    Divergence { step: usize },
    /// The static parser failed where full `LL` went on, without an earlier
    /// divergent choice. `step` is the decision whose static prediction
    /// failed, if a prediction failed.
    StaticFailure { step: Option<usize> },
    /// Full `LL` fails before or where the static parser fails, after the
    /// same choices: the grammar rejects the input.
    GrammarRejects,
    /// The traces disagree in a way that a divergent choice does not
    /// explain, at `step`.
    Inconclusive { step: usize },
}

/// The first divergence inside the neutral parse of a shared-descent group at
/// `step`. A wrong guess in the shared rule can end the rule early or late,
/// so the tail dispatch after it chooses the wrong member: that guess is the
/// root cause, not the group.
fn inner_divergence(
    static_trace: &Trace,
    ll_trace: &Trace,
    step: usize,
    oracle_alt: i32,
) -> Option<usize> {
    let Resolution::Descent {
        alts, neutral_end, ..
    } = &static_trace.events[step].resolution
    else {
        return None;
    };
    if !alts.contains(&oracle_alt) {
        return None;
    }
    let end = (*neutral_end).min(ll_trace.events.len());
    let inner = (step + 1..end).find(|&inner| {
        let (ours, oracle) = (&static_trace.events[inner], &ll_trace.events[inner]);
        ours.decision != oracle.decision
            || ours.token_index != oracle.token_index
            || ours.alt != oracle.alt
    })?;
    let (ours, oracle) = (&static_trace.events[inner], &ll_trace.events[inner]);
    (ours.decision == oracle.decision
        && ours.token_index == oracle.token_index
        && ours.alt.is_some()
        && oracle.alt.is_some())
    .then_some(inner)
}

/// Compares the trace of the static parser with the trace of full `LL`.
pub(super) fn compare(static_trace: &Trace, ll_trace: &Trace) -> Verdict {
    let common = static_trace.events.len().min(ll_trace.events.len());
    for step in 0..common {
        let ours = &static_trace.events[step];
        let oracle = &ll_trace.events[step];
        if ours.decision != oracle.decision || ours.token_index != oracle.token_index {
            return Verdict::Inconclusive { step };
        }
        match (ours.alt, oracle.alt) {
            (Some(our_alt), Some(oracle_alt)) if our_alt != oracle_alt => {
                return Verdict::Divergence {
                    step: inner_divergence(static_trace, ll_trace, step, oracle_alt)
                        .unwrap_or(step),
                };
            }
            // A failed neutral parse of a shared-descent group chose no member
            // yet. The decisions of the shared rule follow in both traces.
            (None, Some(oracle)) if matches!(&ours.resolution, Resolution::DescentAborted { alts, .. } if alts.contains(&oracle)) =>
                {}
            (None, Some(_)) => return Verdict::StaticFailure { step: Some(step) },
            (_, None) => return Verdict::GrammarRejects,
            _ => {}
        }
    }
    let same_length = static_trace.events.len() == ll_trace.events.len();
    match (&static_trace.outcome, &ll_trace.outcome) {
        (Outcome::Accepted { stop }, Outcome::Accepted { stop: oracle_stop })
            if same_length && stop == oracle_stop =>
        {
            Verdict::Agree
        }
        (Outcome::Failed(ours), Outcome::Failed(oracle))
            if same_length && ours.token_index == oracle.token_index =>
        {
            Verdict::GrammarRejects
        }
        (Outcome::Failed(_), _) if static_trace.events.len() <= ll_trace.events.len() => {
            Verdict::StaticFailure { step: None }
        }
        _ => Verdict::Inconclusive { step: common },
    }
}

#[cfg(test)]
mod tests {
    use super::super::interp::{DecisionEvent, ErrorKind, SyntaxError};
    use super::*;

    fn event(decision: usize, token_index: isize, alt: Option<i32>) -> DecisionEvent {
        DecisionEvent {
            decision,
            rule: 0,
            token_index,
            stack: Vec::new(),
            alt,
            resolution: Resolution::Pending,
        }
    }

    fn failed(token_index: isize) -> Outcome {
        Outcome::Failed(SyntaxError {
            kind: ErrorKind::Mismatch,
            token_index,
            expected: Vec::new(),
            stack: Vec::new(),
            step: None,
            message: String::new(),
        })
    }

    fn trace(events: Vec<DecisionEvent>, outcome: Outcome) -> Trace {
        Trace {
            events,
            outcome,
            probe: None,
        }
    }

    #[test]
    fn same_choices_agree() {
        let events = vec![event(0, 0, Some(1)), event(1, 2, Some(2))];
        let ours = trace(events.clone(), Outcome::Accepted { stop: 4 });
        let oracle = trace(events, Outcome::Accepted { stop: 4 });
        assert_eq!(compare(&ours, &oracle), Verdict::Agree);
    }

    #[test]
    fn first_other_choice_is_the_divergence() {
        let ours = trace(
            vec![
                event(0, 0, Some(1)),
                event(1, 2, Some(1)),
                event(2, 3, Some(1)),
            ],
            failed(5),
        );
        let oracle = trace(
            vec![
                event(0, 0, Some(1)),
                event(1, 2, Some(2)),
                event(3, 3, Some(1)),
            ],
            Outcome::Accepted { stop: 6 },
        );
        assert_eq!(compare(&ours, &oracle), Verdict::Divergence { step: 1 });
    }

    #[test]
    fn same_failure_means_the_grammar_rejects() {
        let events = vec![event(0, 0, Some(1))];
        let ours = trace(events.clone(), failed(3));
        let oracle = trace(events, failed(3));
        assert_eq!(compare(&ours, &oracle), Verdict::GrammarRejects);
    }

    #[test]
    fn oracle_failure_at_a_decision_means_the_grammar_rejects() {
        let ours = trace(vec![event(0, 0, Some(1)), event(1, 1, Some(2))], failed(4));
        let oracle = trace(vec![event(0, 0, Some(1)), event(1, 1, None)], failed(1));
        assert_eq!(compare(&ours, &oracle), Verdict::GrammarRejects);
    }

    #[test]
    fn failed_static_prediction_is_a_static_failure() {
        let ours = trace(vec![event(0, 0, Some(1)), event(1, 1, None)], failed(2));
        let oracle = trace(
            vec![event(0, 0, Some(1)), event(1, 1, Some(2))],
            Outcome::Accepted { stop: 3 },
        );
        assert_eq!(
            compare(&ours, &oracle),
            Verdict::StaticFailure { step: Some(1) }
        );
    }

    #[test]
    fn misaligned_traces_are_inconclusive() {
        let ours = trace(vec![event(0, 0, Some(1)), event(1, 1, Some(1))], failed(2));
        let oracle = trace(
            vec![event(0, 0, Some(1)), event(2, 1, Some(1))],
            Outcome::Accepted { stop: 3 },
        );
        assert_eq!(compare(&ours, &oracle), Verdict::Inconclusive { step: 1 });
    }
}
