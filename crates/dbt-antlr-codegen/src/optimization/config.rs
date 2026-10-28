// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::BTreeSet;

use super::options::OptimizationOptions;
use crate::error::EmitError;
use crate::grammar::rule_reachability::EntryRuleConfig;
use crate::grammar::source::SourceSet;
use crate::grammar::transform::passes::inline_trivial::InlineTrivialRules;
use crate::grammar::transform::passes::precedence_ladder::CollapsePrecedenceLadders;
use crate::grammar::transform::passes::prune_unreachable::PruneUnreachableRules;
use crate::grammar::transform::{GrammarTransform, SafetyClass, TransformRegistry};

use super::descriptor::{
    COLLAPSE_PRECEDENCE_LADDERS, INLINE_TRIVIAL_RULES, OptimizationStage, PRUNE_UNREACHABLE_RULES,
    PassDescriptor, STATIC_LEXER_DFA,
};
use super::report::{OptimizationReport, pruned_unreachable_rule_messages};
use super::static_lexer_dfa::StaticLexerDfa;

type MessageRenderer = fn(&OptimizationReport, &SourceSet, &PassDescriptor) -> Vec<String>;

/// One configured pass body, before the plan fixes the pass order.
enum ConfiguredPassStage {
    GrammarTransforms(Box<dyn GrammarTransform>),
    StaticDfaBuilds(StaticDfaPass),
}

/// One static DFA build selected by the plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StaticDfaPass {
    StaticLexerDfa,
}

impl StaticDfaPass {
    pub(super) const fn id(self) -> &'static str {
        match self {
            Self::StaticLexerDfa => StaticLexerDfa::NAME,
        }
    }

    pub(super) const fn safety_class(self) -> SafetyClass {
        match self {
            Self::StaticLexerDfa => StaticLexerDfa::SAFETY,
        }
    }
}

/// The pipeline stage a selected pass runs in.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SelectedPassStage {
    GrammarTransforms,
    StaticDfaBuilds(StaticDfaPass),
}

impl ConfiguredPassStage {
    const fn selected(&self) -> SelectedPassStage {
        match self {
            Self::GrammarTransforms(_) => SelectedPassStage::GrammarTransforms,
            Self::StaticDfaBuilds(pass) => SelectedPassStage::StaticDfaBuilds(*pass),
        }
    }
}

struct ConfiguredPass {
    stage: ConfiguredPassStage,
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
    trivial_inlining: bool,
    precedence_ladders: bool,
    static_lexer_dfa: bool,
    report_only: bool,
}

impl SelectionSettings {
    fn from_options(options: &OptimizationOptions) -> Self {
        Self {
            entry_rules: EntryRuleConfig::new(options.entry_rules.iter().cloned()),
            prune_unreachable: options.prune_unreachable,
            trivial_inlining: options.inline_trivial_rules,
            precedence_ladders: options.optimize_precedence_ladders,
            static_lexer_dfa: options.static_lexer_dfa,
            report_only: options.report_only,
        }
    }
}

pub(super) struct SelectedPass {
    pub(super) descriptor: &'static PassDescriptor,
    pub(super) stage: SelectedPassStage,
    pub(super) emit_manifest: bool,
    pub(super) messages: Option<MessageRenderer>,
}

#[derive(Default)]
pub(crate) struct OptimizationPlan {
    pub(super) entry_rules: EntryRuleConfig,
    pub(super) transforms: TransformRegistry,
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
        Self::from_settings(SelectionSettings::from_options(options))
    }

    fn from_settings(settings: SelectionSettings) -> Result<Self, EmitError> {
        let mut registrations = PASS_REGISTRY.iter().collect::<Vec<_>>();
        registrations.sort_by_key(|registration| registration.descriptor.canonical_order);
        validate_registry(&registrations)?;

        let mut transforms = TransformRegistry::default();
        let mut selected = Vec::new();
        for registration in registrations {
            let Some(configured) = (registration.configure)(&settings) else {
                continue;
            };
            validate_configured_pass(registration.descriptor, &configured)?;
            let stage = configured.stage.selected();
            if let ConfiguredPassStage::GrammarTransforms(transform) = configured.stage {
                transforms.push_boxed(transform);
            }
            selected.push(SelectedPass {
                descriptor: registration.descriptor,
                stage,
                emit_manifest: configured.emit_manifest,
                messages: configured.messages,
            });
        }
        validate_selection(&selected)?;
        Ok(Self {
            entry_rules: settings.entry_rules,
            transforms,
            selected,
            report_only: settings.report_only,
        })
    }

    pub(crate) const fn entry_rules(&self) -> &EntryRuleConfig {
        &self.entry_rules
    }

    pub(crate) const fn transforms(&self) -> &TransformRegistry {
        &self.transforms
    }

    /// The static DFA build passes selected by the plan, in canonical order.
    pub(crate) fn static_dfa_passes(&self) -> impl Iterator<Item = StaticDfaPass> + '_ {
        self.selected.iter().filter_map(|pass| match pass.stage {
            SelectedPassStage::StaticDfaBuilds(pass) => Some(pass),
            SelectedPassStage::GrammarTransforms => None,
        })
    }

    pub(crate) const fn report_only(&self) -> bool {
        self.report_only
    }
}

fn configure_prune_unreachable(settings: &SelectionSettings) -> Option<ConfiguredPass> {
    settings.prune_unreachable.then(|| ConfiguredPass {
        stage: ConfiguredPassStage::GrammarTransforms(Box::new(PruneUnreachableRules::new(
            settings.entry_rules.clone(),
        ))),
        emit_manifest: false,
        messages: Some(pruned_unreachable_rule_messages),
    })
}

fn configure_trivial_inlining(settings: &SelectionSettings) -> Option<ConfiguredPass> {
    settings.trivial_inlining.then(|| ConfiguredPass {
        stage: ConfiguredPassStage::GrammarTransforms(Box::new(InlineTrivialRules::new(
            settings.entry_rules.clone(),
        ))),
        emit_manifest: true,
        messages: None,
    })
}

fn configure_precedence_ladders(settings: &SelectionSettings) -> Option<ConfiguredPass> {
    settings.precedence_ladders.then(|| ConfiguredPass {
        stage: ConfiguredPassStage::GrammarTransforms(Box::new(CollapsePrecedenceLadders)),
        emit_manifest: true,
        messages: None,
    })
}

fn configure_static_lexer_dfa(settings: &SelectionSettings) -> Option<ConfiguredPass> {
    settings.static_lexer_dfa.then_some(ConfiguredPass {
        stage: ConfiguredPassStage::StaticDfaBuilds(StaticDfaPass::StaticLexerDfa),
        emit_manifest: true,
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
        ConfiguredPassStage::GrammarTransforms(transform) => {
            if descriptor.stage != OptimizationStage::GrammarTransforms {
                return Err(invalid_registry(&format!(
                    "pass {} is not a grammar-transform pass",
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
        ConfiguredPassStage::StaticDfaBuilds(pass) => {
            if descriptor.stage != OptimizationStage::StaticDfaBuilds {
                return Err(invalid_registry(&format!(
                    "pass {} is not a static-dfa-build pass",
                    descriptor.id
                )));
            }
            if descriptor.id != pass.id() {
                return Err(invalid_registry(&format!(
                    "descriptor ID {} does not match pass ID {}",
                    descriptor.id,
                    pass.id()
                )));
            }
            if descriptor.safety != pass.safety_class() {
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
        trivial_inlining: bool,
        precedence_ladders: bool,
        static_lexer_dfa: bool,
        report_only: bool,
    ) -> SelectionSettings {
        SelectionSettings {
            entry_rules: EntryRuleConfig::default(),
            prune_unreachable,
            trivial_inlining,
            precedence_ladders,
            static_lexer_dfa,
            report_only,
        }
    }

    #[test]
    fn effective_passes_follow_canonical_descriptor_order() {
        let plan = OptimizationPlan::from_settings(settings(true, true, true, true, false))
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
        assert_eq!(
            plan.static_dfa_passes().collect::<Vec<_>>(),
            [StaticDfaPass::StaticLexerDfa]
        );
        assert!(!plan.report_only());
    }

    #[test]
    fn report_only_preserves_selected_passes_including_static_lexer_dfa() {
        let plan = OptimizationPlan::from_settings(settings(true, false, true, true, true))
            .expect("registered optimization passes should be valid");

        assert!(plan.report_only());
        assert_eq!(
            plan.static_dfa_passes().collect::<Vec<_>>(),
            [StaticDfaPass::StaticLexerDfa]
        );
        assert_eq!(plan.selected.len(), 3);
    }
}
