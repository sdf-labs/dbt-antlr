// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::{BTreeMap, BTreeSet};

use crate::optimization::{
    OptimizationPlan, OptimizationReport, StaticDfaPass, StaticLexerDfa, StaticLexerTables,
    StaticParserDfa,
};

use super::action::{ActionReferenceParser, action_references};
use super::atn::{CompiledLexer, CompiledParser, compile_lexer, compile_parser};
use super::diagnostic::{CompilationError, Diagnostic};
use super::integration::{
    IntegratedGrammarSet, IntegratedVocabularySource, RootOutputs, integrate_loaded,
};
use super::loader::{LoadOptions, LoadedSources, load_recovering};
use super::model::{GrammarId, GrammarKind};
use super::semantics::SemanticGrammarSet;
use super::source::SourceSet;
use super::transform::TransformReport;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CompiledRoot {
    pub(crate) source: GrammarId,
    pub(crate) lexer: Option<GrammarId>,
    pub(crate) parser: Option<GrammarId>,
}

#[derive(Debug)]
pub(crate) struct Compilation {
    pub(crate) sources: SourceSet,
    non_grammar_inputs: Vec<std::path::PathBuf>,
    pub(crate) roots: Vec<CompiledRoot>,
    pub(crate) lexers: BTreeMap<GrammarId, CompiledLexer>,
    pub(crate) parsers: BTreeMap<GrammarId, CompiledParser>,
    static_lexer_tables: BTreeMap<GrammarId, StaticLexerTables>,
    pub(crate) diagnostics: Vec<Diagnostic>,
    optimization_report: OptimizationReport,
}

impl Compilation {
    pub(crate) fn input_paths(&self) -> impl Iterator<Item = &std::path::Path> {
        self.sources.canonical_paths().chain(
            self.non_grammar_inputs
                .iter()
                .map(std::path::PathBuf::as_path),
        )
    }

    pub(crate) fn lexer(&self, grammar: GrammarId) -> Option<&CompiledLexer> {
        self.lexers.get(&grammar)
    }

    pub(crate) fn parser(&self, grammar: GrammarId) -> Option<&CompiledParser> {
        self.parsers.get(&grammar)
    }

    pub(crate) fn static_lexer_tables(&self, grammar: GrammarId) -> Option<&StaticLexerTables> {
        self.static_lexer_tables.get(&grammar)
    }

    pub(crate) const fn optimization_report(&self) -> &OptimizationReport {
        &self.optimization_report
    }

    pub(crate) fn lexer_named(&self, name: &str) -> Option<&CompiledLexer> {
        self.lexers
            .values()
            .find(|compiled| compiled.semantic.unit.name == name)
    }

    pub(crate) fn parser_named(&self, name: &str) -> Option<&CompiledParser> {
        self.parsers
            .values()
            .find(|compiled| compiled.semantic.unit.name == name)
    }
}

pub(crate) fn compile(options: LoadOptions) -> Result<Compilation, CompilationError> {
    compile_with_plan(options, &OptimizationPlan::default())
}

pub(crate) fn compile_with_plan(
    options: LoadOptions,
    plan: &OptimizationPlan,
) -> Result<Compilation, CompilationError> {
    compile_with_action_reference_parser(options, plan, action_references)
}

/// Runs the compile-optimization pipeline.
///
/// The stages always run in this order, and the [`OptimizationPlan`] decides
/// what each optimization stage does:
///
/// 1. [`load_grammar`] loads, integrates, and validates the requested roots.
/// 2. [`grammar_transforms`] runs the selected grammar-transform passes.
/// 3. [`compile_semantics`] compiles and analyzes the transformed grammar.
/// 4. [`static_dfa_builds`] runs the selected static DFA build passes.
pub(crate) fn compile_with_action_reference_parser(
    options: LoadOptions,
    plan: &OptimizationPlan,
    action_reference_parser: ActionReferenceParser,
) -> Result<Compilation, CompilationError> {
    let grammar = load_grammar(options, plan)?;
    let grammar = transform_grammar(grammar, plan, action_reference_parser)?;
    let compilation = compile_semantics(grammar, plan, action_reference_parser)?;
    Ok(static_dfa_builds(compilation, plan))
}

/// The loaded and integrated grammar handed from the load stage to the
/// grammar-transform stage.
struct LoadedGrammar {
    sources: SourceSet,
    non_grammar_inputs: Vec<std::path::PathBuf>,
    root_order: Vec<GrammarId>,
    integrated: IntegratedGrammarSet,
}

/// Loads, integrates, and validates the requested grammar roots.
fn load_grammar(
    options: LoadOptions,
    plan: &OptimizationPlan,
) -> Result<LoadedGrammar, CompilationError> {
    let entry_rules = plan.entry_rules();
    let loaded = load_recovering(options);
    let root_order = loaded.grammars.roots.clone();
    let mut integrated =
        integrate_loaded(&loaded).map_err(|error| error.with_sources(&loaded.sources))?;
    let mut non_grammar_inputs = integrated
        .vocabularies
        .iter()
        .filter_map(|vocabulary| match &vocabulary.source {
            IntegratedVocabularySource::Grammar(_) => None,
            IntegratedVocabularySource::TokensFile(path) => Some(path.clone()),
        })
        .collect::<Vec<_>>();
    non_grammar_inputs.sort();
    non_grammar_inputs.dedup();
    let unknown_entries =
        entry_rules.unknown_names(&integrated.grammar.units, &integrated.grammar.target_units);
    if !unknown_entries.is_empty() {
        let span = integrated
            .grammar
            .units
            .iter()
            .find(|unit| {
                integrated.grammar.target_units.contains(&unit.id)
                    && unit.kind == GrammarKind::Parser
            })
            .or_else(|| integrated.grammar.units.first())
            .map(|unit| unit.span.clone());
        let diagnostics = unknown_entries
            .into_iter()
            .map(|entry| {
                Diagnostic::error_with_optional_span(
                    "G4S079",
                    span.clone(),
                    format!("configured parser entry rule {entry} is not defined"),
                )
            })
            .collect();
        return Err(CompilationError::new(diagnostics).with_sources(&loaded.sources));
    }
    integrated.grammar.preserved_rules =
        entry_rules.matching_rule_ids(&integrated.grammar.units, &integrated.grammar.target_units);
    let LoadedSources { sources, .. } = loaded;
    Ok(LoadedGrammar {
        sources,
        non_grammar_inputs,
        root_order,
        integrated,
    })
}

/// The transformed grammar handed from the grammar-transform stage to the
/// semantic-compilation stage.
struct TransformedGrammar {
    loaded: LoadedGrammar,
    authored_diagnostics: Option<Vec<Diagnostic>>,
    transform_report: TransformReport,
}

/// Runs the grammar-transform passes selected by the plan.
///
/// The authored grammar is analyzed first when transforms are selected, so that
/// semantic diagnostics still point at what the author wrote.
fn transform_grammar(
    mut loaded: LoadedGrammar,
    plan: &OptimizationPlan,
    action_reference_parser: ActionReferenceParser,
) -> Result<TransformedGrammar, CompilationError> {
    let entry_rules = plan.entry_rules();
    let transforms = plan.transforms();
    let authored_diagnostics = if transforms.is_empty() {
        None
    } else {
        Some(
            super::semantics::analyze_with_action_reference_parser(
                &loaded.sources,
                loaded.integrated.clone(),
                entry_rules,
                action_reference_parser,
            )
            .map_err(|error| error.with_sources(&loaded.sources))?
            .diagnostics,
        )
    };
    // Report-only runs still apply the transforms: the later stages must
    // analyze the real result, and only file emission is suppressed.
    let transform_report = transforms
        .run_with_action_reference_parser(
            &mut loaded.integrated.grammar,
            &mut loaded.integrated.ids,
            false,
            action_reference_parser,
        )
        .map_err(|diagnostic| {
            CompilationError::new(vec![diagnostic]).with_sources(&loaded.sources)
        })?;
    Ok(TransformedGrammar {
        loaded,
        authored_diagnostics,
        transform_report,
    })
}

fn merge_diagnostics(
    mut authored: Vec<Diagnostic>,
    transformed: Vec<Diagnostic>,
) -> Vec<Diagnostic> {
    let mut matched = vec![false; authored.len()];
    for diagnostic in transformed {
        let existing = authored.iter().enumerate().find_map(|(index, candidate)| {
            (!matched[index] && candidate == &diagnostic).then_some(index)
        });
        if let Some(index) = existing {
            matched[index] = true;
        } else {
            authored.push(diagnostic);
            matched.push(true);
        }
    }
    authored
}

/// Compiles the transformed grammar into recognizer artifacts.
fn compile_semantics(
    transformed: TransformedGrammar,
    plan: &OptimizationPlan,
    action_reference_parser: ActionReferenceParser,
) -> Result<Compilation, CompilationError> {
    let TransformedGrammar {
        loaded,
        authored_diagnostics,
        transform_report,
    } = transformed;
    let LoadedGrammar {
        sources,
        non_grammar_inputs,
        root_order,
        integrated,
    } = loaded;
    let mut semantics = super::semantics::analyze_with_action_reference_parser(
        &sources,
        integrated,
        plan.entry_rules(),
        action_reference_parser,
    )
    .map_err(|error| error.with_sources(&sources))?;
    if let Some(authored_diagnostics) = authored_diagnostics {
        semantics.diagnostics = merge_diagnostics(
            authored_diagnostics,
            std::mem::take(&mut semantics.diagnostics),
        );
    }
    let SemanticGrammarSet {
        grammars,
        roots,
        diagnostics: mut all_diagnostics,
        provenance,
        ..
    } = semantics;
    let roots: Vec<CompiledRoot> = root_order
        .into_iter()
        .map(|source| {
            let RootOutputs { lexer, parser } = roots[&source];
            CompiledRoot {
                source,
                lexer,
                parser,
            }
        })
        .collect();
    let mut lexers = BTreeMap::new();
    let mut parsers = BTreeMap::new();
    for grammar in grammars {
        let grammar_id = grammar.unit.id;
        match grammar.unit.kind {
            GrammarKind::Lexer => {
                let compiled = compile_lexer(grammar, provenance.clone())
                    .map_err(|error| error.with_sources(&sources))?;
                all_diagnostics.extend(compiled.analysis.diagnostics.iter().cloned());
                lexers.insert(grammar_id, compiled);
            }
            GrammarKind::Parser => {
                let compiled = compile_parser(grammar, provenance.clone())
                    .map_err(|error| error.with_sources(&sources))?;
                all_diagnostics.extend(compiled.analysis.diagnostics.iter().cloned());
                parsers.insert(grammar_id, compiled);
            }
            GrammarKind::Combined => {
                unreachable!("combined grammars are split before semantic analysis")
            }
        }
    }
    Ok(Compilation {
        sources,
        non_grammar_inputs,
        roots,
        lexers,
        parsers,
        static_lexer_tables: BTreeMap::new(),
        diagnostics: all_diagnostics,
        optimization_report: OptimizationReport::new(transform_report),
    })
}

/// Runs the static DFA build passes selected by the plan.
///
/// The stage runs after semantic compilation, so the pass diagnostics it
/// appends follow the lexer and parser diagnostics.
fn static_dfa_builds(mut compilation: Compilation, plan: &OptimizationPlan) -> Compilation {
    for pass in plan.static_dfa_passes() {
        match pass {
            StaticDfaPass::StaticLexerDfa => build_static_lexer_dfas(&mut compilation),
            StaticDfaPass::StaticParserDfa => build_static_parser_dfas(&mut compilation),
        }
    }
    compilation
}

/// Builds the static lexer DFA tables of every root lexer and records their
/// diagnostics and optimization reports.
fn build_static_lexer_dfas(compilation: &mut Compilation) {
    let lexer_ids = compilation
        .roots
        .iter()
        .filter_map(|root| root.lexer)
        .collect::<BTreeSet<_>>();
    for lexer_id in lexer_ids {
        let Some(compiled) = compilation.lexers.get(&lexer_id) else {
            continue;
        };
        let build = StaticLexerDfa::build(compiled);
        compilation.diagnostics.extend(build.diagnostics);
        compilation
            .optimization_report
            .push_static_lexer_dfa(build.report);
        compilation
            .static_lexer_tables
            .insert(lexer_id, build.tables);
    }
}

/// Inventories the decisions of every root parser and records the pass report.
fn build_static_parser_dfas(compilation: &mut Compilation) {
    let parser_ids = compilation
        .roots
        .iter()
        .filter_map(|root| root.parser)
        .collect::<BTreeSet<_>>();
    for parser_id in parser_ids {
        let Some(compiled) = compilation.parsers.get(&parser_id) else {
            continue;
        };
        let report = StaticParserDfa::report(compiled);
        compilation
            .optimization_report
            .push_static_parser_dfa(report);
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::OptimizationOptions;

    use super::super::diagnostic::Severity;
    use super::*;

    #[test]
    fn combined_root_owns_shared_direct_artifacts() {
        let compilation = compile(LoadOptions {
            roots: vec![fixture("vscode-sentences").join("sentences.g4")],
            library_directories: Vec::new(),
        })
        .expect("combined fixture should compile");

        let [root] = compilation.roots.as_slice() else {
            panic!("one requested root should produce one compiled root");
        };
        let lexer_id = root.lexer.expect("combined root has a lexer");
        let lexer = compilation
            .lexer(lexer_id)
            .expect("root lexer artifact exists");
        let parser = compilation
            .parser(root.parser.expect("combined root has a parser"))
            .expect("root parser artifact exists");
        assert_eq!(lexer.semantic.unit.name, "sentencesLexer");
        assert_eq!(parser.semantic.unit.name, "sentencesParser");
        assert!(compilation.static_lexer_tables(lexer_id).is_none());
        assert_eq!(compilation.sources.len(), 1);
        assert_eq!(compilation.optimization_report().transforms.entries, []);
    }

    #[test]
    fn static_lexer_dfa_plan_populates_static_lexer_and_diagnostics() {
        let mut options = OptimizationOptions::new();
        options.static_lexer_dfa(true);
        let plan = OptimizationPlan::from_options(&options).expect("plan should be valid");
        let compilation = compile_with_plan(
            LoadOptions {
                roots: vec![fixture("lexer-recursion").join("LexerRecursion.g4")],
                library_directories: Vec::new(),
            },
            &plan,
        )
        .expect("lexer fixture should compile");

        let [root] = compilation.roots.as_slice() else {
            panic!("one requested root should produce one compiled root");
        };
        let lexer_id = root.lexer.expect("lexer root has a lexer");
        let tables = compilation
            .static_lexer_tables(lexer_id)
            .expect("static lexer tables built");
        assert!(!tables.words.is_empty());
        assert_eq!(compilation.optimization_report().static_lexer_dfa.len(), 1);
        assert!(
            compilation
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "G4S188")
        );
    }

    #[test]
    fn static_parser_dfa_plan_records_the_decision_inventory() {
        let mut options = OptimizationOptions::new();
        options.static_parser_dfa(true);
        let plan = OptimizationPlan::from_options(&options).expect("plan should be valid");
        let compilation = compile_with_plan(
            LoadOptions {
                roots: vec![fixture("parser-shapes").join("ParserShapes.g4")],
                library_directories: Vec::new(),
            },
            &plan,
        )
        .expect("parser fixture should compile");

        let [root] = compilation.roots.as_slice() else {
            panic!("one requested root should produce one compiled root");
        };
        let parser = compilation
            .parser(root.parser.expect("parser root has a parser"))
            .expect("root parser artifact exists");
        let [report] = compilation
            .optimization_report()
            .static_parser_dfa
            .as_slice()
        else {
            panic!("the parser pass records one report");
        };
        assert_eq!(report.grammar, "ParserShapes");
        assert_eq!(report.decisions.len(), parser.packed.decision_count());
        assert!(
            report
                .decisions
                .iter()
                .any(|decision| decision.rule == "optional"),
            "the inventory names the decisions by rule: {:?}",
            report.decisions
        );
        assert!(!report.vocabulary.is_empty());
    }
    #[test]
    fn missing_token_vocab_matches_java_diagnostic_site() {
        let error = compile_fixture("vscode-split-errors", &["TLexer2.g4", "TParser2.g4"])
            .expect_err("missing token vocabulary must be fatal");
        assert_diagnostic(
            &error,
            "G4L007",
            Severity::Error,
            "TLexer2.g4",
            Some((4, 14)),
            "cannot find token vocabulary nonexisting",
        );
    }

    #[test]
    fn implicit_token_warning_precedes_unknown_channel_error() {
        let error = compile_fixture("vscode-diagnostics", &["t.g4"])
            .expect_err("unknown channel must be fatal");
        assert_diagnostic(
            &error,
            "G4S030",
            Severity::Warning,
            "t.g4",
            Some((3, 3)),
            "implicit definition of token ZZ in parser",
        );
        assert_diagnostic(
            &error,
            "G4S053",
            Severity::Error,
            "t.g4",
            Some((8, 18)),
            "BLAH is not a recognized channel",
        );
    }

    #[test]
    fn indirect_left_recursion_matches_java_cycle_members() {
        let error = compile_fixture("vscode-indirect-left-recursion", &["t2.g4"])
            .expect_err("mutual left recursion must be fatal");
        assert_diagnostic(
            &error,
            "G4A005",
            Severity::Error,
            "t2.g4",
            None,
            "mutually left-recursive rules: [a, c, b]",
        );
    }

    fn compile_fixture(
        fixture_name: &str,
        roots: &[&str],
    ) -> Result<Compilation, CompilationError> {
        let directory = fixture(fixture_name);
        compile(LoadOptions {
            roots: roots.iter().map(|root| directory.join(root)).collect(),
            library_directories: Vec::new(),
        })
    }

    fn assert_diagnostic(
        error: &CompilationError,
        code: &str,
        severity: Severity,
        source_name: &str,
        position: Option<(usize, usize)>,
        message: &str,
    ) {
        let diagnostic = error
            .diagnostics()
            .iter()
            .find(|diagnostic| diagnostic.code == code)
            .unwrap_or_else(|| panic!("missing {code} diagnostic: {error:#?}"));
        assert_eq!(diagnostic.severity, severity);
        assert_eq!(diagnostic.message, message);
        if let Some((line, column)) = position {
            let text = std::fs::read_to_string(
                fixture_directory_for_source(source_name)
                    .unwrap_or_else(|| panic!("unknown fixture source {source_name}")),
            )
            .expect("fixture source should be readable");
            assert_eq!(
                diagnostic.primary.bytes.start,
                byte_offset(&text, line, column),
                "{source_name}:{line}:{column}",
            );
        }
    }

    fn fixture_directory_for_source(source_name: &str) -> Option<PathBuf> {
        [
            "vscode-split-errors",
            "vscode-diagnostics",
            "vscode-indirect-left-recursion",
        ]
        .into_iter()
        .map(|fixture_name| fixture(fixture_name).join(source_name))
        .find(|path| path.is_file())
    }

    fn byte_offset(text: &str, line: usize, column: usize) -> u32 {
        let line_start = text
            .split_inclusive('\n')
            .take(line.saturating_sub(1))
            .map(str::len)
            .sum::<usize>();
        let byte_column = text[line_start..]
            .chars()
            .take(column)
            .map(char::len_utf8)
            .sum::<usize>();
        u32::try_from(line_start + byte_column).expect("fixture offset exceeds u32")
    }

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/codegen-direct/fixtures")
            .join(name)
    }
}
