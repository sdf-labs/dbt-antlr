// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::BTreeSet;

use super::options::OptimizationOptions;
use crate::error::EmitError;
use crate::grammar::compiler::Compilation;
use crate::grammar::rule_reachability::EntryRuleConfig;
use crate::grammar::transform::passes::inline_trivial::InlineTrivialRules;
use crate::grammar::transform::passes::precedence_ladder::CollapsePrecedenceLadders;
use crate::grammar::transform::passes::prune_unreachable::PruneUnreachableRules;
use crate::grammar::transform::{GrammarTransform, TransformRegistry};

use super::descriptor::{
    COLLAPSE_PRECEDENCE_LADDERS, INLINE_TRIVIAL_RULES, OptimizationStage, PRUNE_UNREACHABLE_RULES,
    PassDescriptor, STATIC_LEXER_DFA,
};
use super::report::pruned_unreachable_rule_messages;
use super::static_lexer_dfa::StaticLexerDfa;

type MessageRenderer = fn(&Compilation, &PassDescriptor) -> Vec<String>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PassExecution {
    Apply,
    ReportOnly,
}

enum ConfiguredPassStage {
    IntegratedGrammar(Box<dyn GrammarTransform>),
    StaticLexerDfa,
}

struct ConfiguredPass {
    stage: ConfiguredPassStage,
    execution: PassExecution,
    emit_manifest: bool,
    messages: Option<MessageRenderer>,
}

struct PassRegistration {
    descriptor: &'static PassDescriptor,
    configure: fn(&SelectionSettings) -> Option<ConfiguredPass>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct SelectionSettings {
    entry_rules: EntryRuleConfig,
    prune_unreachable: bool,
    trivial_inlining: Option<PassExecution>,
    precedence_ladders: Option<PassExecution>,
    static_lexer_dfa: bool,
}

impl SelectionSettings {
    fn from_options(options: &OptimizationOptions) -> Result<Self, EmitError> {
        if options.inline_trivial_rules && options.report_trivial_rules {
            return Err(EmitError::Config(
                "--inline-trivial-rules conflicts with --report-trivial-rules".to_owned(),
            ));
        }
        if options.optimize_precedence_ladders && options.report_precedence_ladders {
            return Err(EmitError::Config(
                "--optimize-precedence-ladders conflicts with --report-precedence-ladders"
                    .to_owned(),
            ));
        }
        let trivial_inlining = if options.report_trivial_rules {
            Some(PassExecution::ReportOnly)
        } else if options.inline_trivial_rules {
            Some(PassExecution::Apply)
        } else {
            None
        };
        let precedence_ladders = if options.report_precedence_ladders {
            Some(PassExecution::ReportOnly)
        } else if options.optimize_precedence_ladders {
            Some(PassExecution::Apply)
        } else {
            None
        };
        Ok(Self {
            entry_rules: EntryRuleConfig::new(options.entry_rules.iter().cloned()),
            prune_unreachable: options.prune_unreachable,
            trivial_inlining,
            precedence_ladders,
            static_lexer_dfa: options.static_lexer_dfa,
        })
    }
}

pub(super) struct SelectedPass {
    pub(super) descriptor: &'static PassDescriptor,
    pub(super) execution: PassExecution,
    pub(super) emit_manifest: bool,
    pub(super) messages: Option<MessageRenderer>,
}

#[derive(Default)]
pub(crate) struct OptimizationPlan {
    pub(super) entry_rules: EntryRuleConfig,
    pub(super) transforms: TransformRegistry,
    pub(super) static_lexer_dfa: bool,
    pub(super) selected: Vec<SelectedPass>,
    pub(super) report_only: bool,
}

const PASS_REGISTRY: &[PassRegistration] = &[
    PassRegistration {
        descriptor: &PRUNE_UNREACHABLE_RULES,
        configure: configure_prune_unreachable,
    },
    PassRegistration {
        descriptor: &INLINE_TRIVIAL_RULES,
        configure: configure_trivial_inlining,
    },
    PassRegistration {
        descriptor: &COLLAPSE_PRECEDENCE_LADDERS,
        configure: configure_precedence_ladders,
    },
    PassRegistration {
        descriptor: &STATIC_LEXER_DFA,
        configure: configure_static_lexer_dfa,
    },
];

impl OptimizationPlan {
    pub(crate) fn from_options(options: &OptimizationOptions) -> Result<Self, EmitError> {
        Self::from_settings(SelectionSettings::from_options(options)?)
    }

    fn from_settings(settings: SelectionSettings) -> Result<Self, EmitError> {
        let mut registrations = PASS_REGISTRY.iter().collect::<Vec<_>>();
        registrations.sort_by_key(|registration| registration.descriptor.canonical_order);
        validate_registry(&registrations)?;

        let mut transforms = TransformRegistry::default();
        let mut static_lexer_dfa = false;
        let mut selected = Vec::new();
        for registration in registrations {
            let Some(configured) = (registration.configure)(&settings) else {
                continue;
            };
            validate_configured_pass(registration.descriptor, &configured)?;
            match configured.stage {
                ConfiguredPassStage::IntegratedGrammar(transform) => {
                    transforms.push_boxed(transform);
                }
                ConfiguredPassStage::StaticLexerDfa => {
                    static_lexer_dfa = configured.execution == PassExecution::Apply;
                }
            }
            selected.push(SelectedPass {
                descriptor: registration.descriptor,
                execution: configured.execution,
                emit_manifest: configured.emit_manifest,
                messages: configured.messages,
            });
        }
        validate_selection(&selected)?;
        let report_only = selected
            .iter()
            .any(|pass| pass.execution == PassExecution::ReportOnly);
        Ok(Self {
            entry_rules: settings.entry_rules,
            transforms,
            static_lexer_dfa: static_lexer_dfa && !report_only,
            selected,
            report_only,
        })
    }

    pub(crate) const fn entry_rules(&self) -> &EntryRuleConfig {
        &self.entry_rules
    }

    pub(crate) const fn transforms(&self) -> &TransformRegistry {
        &self.transforms
    }

    pub(crate) const fn static_lexer_dfa(&self) -> bool {
        self.static_lexer_dfa
    }

    pub(crate) const fn report_only(&self) -> bool {
        self.report_only
    }
}

fn configure_prune_unreachable(settings: &SelectionSettings) -> Option<ConfiguredPass> {
    settings.prune_unreachable.then(|| ConfiguredPass {
        stage: ConfiguredPassStage::IntegratedGrammar(Box::new(PruneUnreachableRules::new(
            settings.entry_rules.clone(),
        ))),
        execution: PassExecution::Apply,
        emit_manifest: false,
        messages: Some(pruned_unreachable_rule_messages),
    })
}

fn configure_trivial_inlining(settings: &SelectionSettings) -> Option<ConfiguredPass> {
    settings.trivial_inlining.map(|execution| ConfiguredPass {
        stage: ConfiguredPassStage::IntegratedGrammar(Box::new(InlineTrivialRules::new(
            settings.entry_rules.clone(),
        ))),
        execution,
        emit_manifest: true,
        messages: None,
    })
}

fn configure_precedence_ladders(settings: &SelectionSettings) -> Option<ConfiguredPass> {
    settings.precedence_ladders.map(|execution| ConfiguredPass {
        stage: ConfiguredPassStage::IntegratedGrammar(Box::new(CollapsePrecedenceLadders)),
        execution,
        emit_manifest: true,
        messages: None,
    })
}

fn configure_static_lexer_dfa(settings: &SelectionSettings) -> Option<ConfiguredPass> {
    settings.static_lexer_dfa.then_some(ConfiguredPass {
        stage: ConfiguredPassStage::StaticLexerDfa,
        execution: PassExecution::Apply,
        emit_manifest: false,
        messages: None,
    })
}

fn validate_registry(registrations: &[&PassRegistration]) -> Result<(), EmitError> {
    let mut ids = BTreeSet::new();
    let mut orders = BTreeSet::new();
    for registration in registrations {
        let descriptor = registration.descriptor;
        if !ids.insert(descriptor.id) {
            return Err(invalid_registry(&format!(
                "duplicate stable pass ID {}",
                descriptor.id
            )));
        }
        if !orders.insert(descriptor.canonical_order) {
            return Err(invalid_registry(&format!(
                "duplicate canonical order {}",
                descriptor.canonical_order
            )));
        }
    }
    for registration in registrations {
        let descriptor = registration.descriptor;
        for related in descriptor.prerequisites.iter().chain(descriptor.conflicts) {
            if !ids.contains(related) {
                return Err(invalid_registry(&format!(
                    "pass {} references unknown pass {related}",
                    descriptor.id
                )));
            }
        }
    }
    Ok(())
}

fn validate_configured_pass(
    descriptor: &PassDescriptor,
    configured: &ConfiguredPass,
) -> Result<(), EmitError> {
    match &configured.stage {
        ConfiguredPassStage::IntegratedGrammar(transform) => {
            if descriptor.stage != OptimizationStage::IntegratedGrammar {
                return Err(invalid_registry(&format!(
                    "pass {} is not an integrated-grammar pass",
                    descriptor.id
                )));
            }
            if descriptor.id != transform.name() {
                return Err(invalid_registry(&format!(
                    "descriptor ID {} does not match transform ID {}",
                    descriptor.id,
                    transform.name()
                )));
            }
            if descriptor.safety != transform.safety_class() {
                return Err(invalid_registry(&format!(
                    "descriptor safety for {} does not match its transform",
                    descriptor.id
                )));
            }
        }
        ConfiguredPassStage::StaticLexerDfa => {
            if descriptor.stage != OptimizationStage::StaticLexerDfa {
                return Err(invalid_registry(&format!(
                    "pass {} is not a static-lexer-dfa pass",
                    descriptor.id
                )));
            }
            if descriptor.id != StaticLexerDfa::NAME {
                return Err(invalid_registry(&format!(
                    "descriptor ID {} does not match pass ID {}",
                    descriptor.id,
                    StaticLexerDfa::NAME
                )));
            }
            if descriptor.safety != StaticLexerDfa::SAFETY {
                return Err(invalid_registry(&format!(
                    "descriptor safety for {} does not match its pass",
                    descriptor.id
                )));
            }
        }
    }
    Ok(())
}

fn validate_selection(selected: &[SelectedPass]) -> Result<(), EmitError> {
    let ids = selected
        .iter()
        .map(|pass| pass.descriptor.id)
        .collect::<BTreeSet<_>>();
    for pass in selected {
        for prerequisite in pass.descriptor.prerequisites {
            if !ids.contains(prerequisite) {
                return Err(EmitError::Config(format!(
                    "optimization pass {} requires {prerequisite}",
                    pass.descriptor.id
                )));
            }
        }
        for conflict in pass.descriptor.conflicts {
            if ids.contains(conflict) {
                return Err(EmitError::Config(format!(
                    "optimization pass {} conflicts with {conflict}",
                    pass.descriptor.id
                )));
            }
        }
    }
    Ok(())
}

fn invalid_registry(message: &str) -> EmitError {
    EmitError::Config(format!("invalid optimization pass registry: {message}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(
        prune_unreachable: bool,
        trivial_inlining: Option<PassExecution>,
        precedence_ladders: Option<PassExecution>,
        static_lexer_dfa: bool,
    ) -> SelectionSettings {
        SelectionSettings {
            entry_rules: EntryRuleConfig::default(),
            prune_unreachable,
            trivial_inlining,
            precedence_ladders,
            static_lexer_dfa,
        }
    }

    #[test]
    fn effective_passes_follow_canonical_descriptor_order() {
        let plan = OptimizationPlan::from_settings(settings(
            true,
            Some(PassExecution::Apply),
            Some(PassExecution::Apply),
            true,
        ))
        .expect("registered optimization passes should be valid");

        assert_eq!(
            plan.selected
                .iter()
                .map(|pass| (pass.descriptor.id, pass.descriptor.canonical_order))
                .collect::<Vec<_>>(),
            [
                (PruneUnreachableRules::NAME, 100),
                (InlineTrivialRules::NAME, 150),
                (CollapsePrecedenceLadders::NAME, 200),
                (StaticLexerDfa::NAME, 300),
            ]
        );
        assert!(plan.static_lexer_dfa());
        assert!(!plan.report_only());
    }

    #[test]
    fn report_only_selection_is_effective_for_the_whole_transform_projection() {
        let plan = OptimizationPlan::from_settings(settings(
            true,
            None,
            Some(PassExecution::ReportOnly),
            true,
        ))
        .expect("registered optimization passes should be valid");

        assert!(plan.report_only());
        assert!(!plan.static_lexer_dfa());
        assert_eq!(plan.selected.len(), 3);
    }

    #[test]
    fn report_only_trivial_inlining_shadows_the_whole_selection() {
        let plan = OptimizationPlan::from_settings(settings(
            false,
            Some(PassExecution::ReportOnly),
            None,
            false,
        ))
        .expect("registered optimization passes should be valid");

        assert!(plan.report_only());
        assert_eq!(
            plan.selected
                .iter()
                .map(|pass| pass.descriptor.id)
                .collect::<Vec<_>>(),
            [InlineTrivialRules::NAME]
        );
    }

    #[test]
    fn apply_and_report_modes_of_one_transform_conflict() {
        let options = OptimizationOptions {
            inline_trivial_rules: true,
            report_trivial_rules: true,
            ..OptimizationOptions::default()
        };
        assert!(OptimizationPlan::from_options(&options).is_err());
        let options = OptimizationOptions {
            optimize_precedence_ladders: true,
            report_precedence_ladders: true,
            ..OptimizationOptions::default()
        };
        assert!(OptimizationPlan::from_options(&options).is_err());
    }
}
