// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--explain`: the differential fault localizer of static parser DFAs.
//!
//! A `--static-dfa-only` parser can reject input that the grammar accepts:
//! where the static analysis could not decide a decision, the table guesses.
//! A wrong guess shows only later, as a syntax error some tokens after the
//! decision that caused it.
//!
//! The explanation parses one input twice, with the compiled ATN and the
//! static tables in memory:
//!
//! 1. as the generated static parser predicts (see [`interp::Oracle::Static`]),
//! 2. with full `LL` prediction, which is exact.
//!
//! It then compares the two decision traces. The first decision where the
//! static parser chose another alternative is the root cause. An SLL
//! prediction at that decision tells a lookahead budget cutoff (SLL agrees
//! with `LL`) from context sensitivity (SLL agrees with the static guess).
//! When full `LL` also fails at the same place, the grammar itself rejects
//! the input.

mod diff;
mod interp;
mod lex;
mod render;
#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager;
use dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables as RuntimeTables;

use crate::emission::compile_grammar_with;
use crate::error::EmitError;
use crate::grammar::atn::{CompiledLexer, CompiledParser};
use crate::grammar::compiler::Compilation;
use crate::optimization::{
    AtnFallback, OptimizationOptions, OptimizationPlan, StaticParserDfaReport,
};

use interp::{Oracle, ParserSpec, StaticOracle};
use lex::{LexerSpec, leak_str, leak_strings};

/// One input to explain against a compiled grammar.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ExplainCase {
    /// The input text.
    pub(crate) input: String,
    /// How the report names the input.
    pub(crate) input_name: String,
    /// Optional start rule override for this input.
    pub(crate) start_rule: Option<String>,
}

/// One `--explain` invocation, with one or more inputs.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ExplainRequest {
    /// Inputs to explain in order.
    pub(crate) cases: Vec<ExplainCase>,
    /// Default parser rule to start from. The first rule is the fallback.
    pub(crate) start_rule: Option<String>,
    /// A lexer grammar that tokenizes the input, for a parser grammar whose
    /// lexer the compilation does not load.
    pub(crate) lexer: Option<PathBuf>,
    /// Whether to include verbose sections (syntax error details and static
    /// guesses) in the report.
    pub(crate) verbose: bool,
}

/// What an `--explain` run prints.
#[derive(Debug)]
pub(crate) struct ExplainOutput {
    pub(crate) diagnostics: Vec<crate::Diagnostic>,
    pub(crate) optimization_messages: Vec<String>,
    /// The stdout reports: the reports the options select, then the
    /// explanation.
    pub(crate) reports: Vec<String>,
}

/// Compiles the grammar at `grammar_path` once and explains how its static
/// parser handles every case in `request.cases`.
pub(crate) fn explain(
    grammar_path: &Path,
    library_dirs: &[PathBuf],
    options: &OptimizationOptions,
    request: &ExplainRequest,
) -> Result<ExplainOutput, EmitError> {
    if !options.static_parser_dfa && !options.static_dfa_only {
        return Err(EmitError::Config(
            "--explain compares a static DFA parser with full LL; add --static-dfa-only or --static-parser-dfa".to_owned(),
        ));
    }
    if request.cases.is_empty() {
        return Err(EmitError::Config(
            "--explain needs at least one input".to_owned(),
        ));
    }
    let mut options = options.clone();
    options.report_only(true);

    let plan = OptimizationPlan::from_options(&options)?;
    // The directory of `--lexer` also resolves the `tokenVocab` of the parser.
    let mut library_dirs = library_dirs.to_vec();
    if let Some(parent) = request.lexer.as_deref().and_then(Path::parent) {
        library_dirs.push(parent.to_path_buf());
    }
    let compilation = compile_grammar_with(grammar_path, &library_dirs, &plan)?;
    let mut diagnostics = crate::compile_error::non_fatal_diagnostics(&compilation);
    let optimization_messages =
        plan.diagnostic_messages(compilation.optimization_report(), &compilation.sources);
    let rendered = plan.rendered_reports(compilation.optimization_report());
    diagnostics.extend(rendered.diagnostics.iter().map(|diagnostic| {
        crate::compile_error::public_diagnostic(diagnostic, &compilation.sources)
    }));
    let mut reports = rendered.reports;

    let lexer_compilation = match &request.lexer {
        Some(path) => Some(compile_grammar_with(
            path,
            &library_dirs,
            &OptimizationPlan::default(),
        )?),
        None => None,
    };
    let (parser, lexer) = units(&compilation, lexer_compilation.as_ref())?;
    let session = ExplainSession::new(&compilation, parser, lexer, plan.atn_fallback())?;
    let report = session.explain_request(request)?;
    reports.push(report);
    Ok(ExplainOutput {
        diagnostics,
        optimization_messages,
        reports,
    })
}

/// The parser unit to explain and the lexer that tokenizes its input.
fn units<'c>(
    compilation: &'c Compilation,
    lexer_compilation: Option<&'c Compilation>,
) -> Result<(&'c CompiledParser, &'c CompiledLexer), EmitError> {
    let root = compilation
        .roots
        .iter()
        .find(|root| root.parser.is_some())
        .ok_or_else(|| {
            EmitError::Config("--explain needs a parser or combined grammar".to_owned())
        })?;
    let parser = root
        .parser
        .and_then(|parser| compilation.parser(parser))
        .ok_or_else(|| EmitError::Config("the parser grammar did not compile".to_owned()))?;
    if let Some(lexer_compilation) = lexer_compilation {
        return lexer_compilation
            .roots
            .iter()
            .find_map(|root| root.lexer.and_then(|lexer| lexer_compilation.lexer(lexer)))
            .map(|lexer| (parser, lexer))
            .ok_or_else(|| {
                EmitError::Config("--lexer needs a lexer or combined grammar".to_owned())
            });
    }
    let token_vocab = parser
        .semantic
        .unit
        .options
        .iter()
        .find(|option| option.name.value == "tokenVocab")
        .map(|option| option.value.value.as_str());
    root.lexer
        .and_then(|lexer| compilation.lexer(lexer))
        .or_else(|| token_vocab.and_then(|name| compilation.lexer_named(name)))
        .map(|lexer| (parser, lexer))
        .ok_or_else(|| {
            EmitError::Config(format!(
                "cannot find the lexer of parser grammar {}; pass its lexer grammar with --lexer <Lexer.g4>",
                parser.semantic.unit.name
            ))
        })
}

struct Explained {
    report: String,
    /// The static trace, the full `LL` trace, and their comparison; `None`
    /// when the lexer rejected the input.
    #[cfg_attr(not(test), allow(dead_code))]
    traces: Option<(interp::Trace, interp::Trace, diff::Verdict)>,
}

/// Prepared static and full-LL interpreters for explaining one or more inputs
/// against a single compiled grammar.
struct ExplainSession<'c> {
    rule_names: &'c [String],
    report: &'c StaticParserDfaReport,
    mode: &'static str,
    lexer_spec: LexerSpec,
    spec: ParserSpec,
    static_oracle: Oracle<'c>,
    static_manager: &'static ParserATNSimulatorManager,
    ll_manager: &'static ParserATNSimulatorManager,
    sll_manager: &'static ParserATNSimulatorManager,
}

impl<'c> ExplainSession<'c> {
    fn new(
        compilation: &'c Compilation,
        parser: &'c CompiledParser,
        lexer: &'c CompiledLexer,
        fallback: AtnFallback,
    ) -> Result<Self, EmitError> {
        let recognizer = &parser.semantic.recognizer;
        let report = compilation
            .optimization_report()
            .static_parser_dfa
            .iter()
            .find(|report| report.grammar == parser.semantic.unit.name)
            .ok_or_else(|| {
                EmitError::Config("the static parser DFA pass did not run".to_owned())
            })?;
        let tables = compilation
            .static_parser_tables(parser.semantic.unit.id)
            .ok_or_else(|| {
                EmitError::Config("the static parser DFA pass built no tables".to_owned())
            })?;
        let words = tables.words();
        let runtime: &'static RuntimeTables = if words.is_empty() {
            Box::leak(Box::default())
        } else {
            let words: &'static [u32] = Box::leak(words.into_boxed_slice());
            Box::leak(Box::new(RuntimeTables::from_image(words).map_err(
                |error| EmitError::Config(format!("invalid static parser DFA tables: {error}")),
            )?))
        };
        let results = report
            .decisions
            .iter()
            .map(|decision| decision.result.clone())
            .collect::<Vec<_>>();
        let mut call_sites = HashMap::new();
        for decision in 0..results.len() {
            if let Some(plan) = tables.descent(decision) {
                for group in &plan.groups {
                    call_sites.insert(
                        (decision, group.virtual_alt),
                        Box::leak(group.call_sites.clone().into_boxed_slice()) as &'static [i32],
                    );
                }
            }
        }
        let spec = ParserSpec {
            grammar_file_name: leak_str(&recognizer.name),
            rule_names: leak_strings(&recognizer.rule_names),
            literal_names: recognizer.literal_names.clone(),
            symbolic_names: recognizer.symbolic_names.clone(),
            atn: parser.atn,
        };
        let static_manager: &'static ParserATNSimulatorManager = Box::leak(Box::new(
            ParserATNSimulatorManager::new_with_static_parser_tables(spec.atn, runtime),
        ));
        let ll_manager: &'static ParserATNSimulatorManager =
            Box::leak(Box::new(ParserATNSimulatorManager::new(spec.atn)));
        let sll_manager: &'static ParserATNSimulatorManager =
            Box::leak(Box::new(ParserATNSimulatorManager::new(spec.atn)));
        let static_oracle = Oracle::Static(StaticOracle {
            results,
            tables,
            ll1: ll1_decisions(parser),
            call_sites,
            static_only: fallback == AtnFallback::Forbidden,
        });
        Ok(Self {
            rule_names: &recognizer.rule_names,
            report,
            mode: match fallback {
                AtnFallback::Forbidden => "--static-dfa-only",
                AtnFallback::Allowed => "--static-parser-dfa",
            },
            lexer_spec: LexerSpec::new(lexer),
            spec,
            static_oracle,
            static_manager,
            ll_manager,
            sll_manager,
        })
    }

    fn resolve_start_rule(&self, name: Option<&str>) -> Result<(usize, &'c str), EmitError> {
        let index = match name {
            Some(name) => self
                .rule_names
                .iter()
                .position(|rule| rule == name)
                .ok_or_else(|| {
                    EmitError::Config(format!(
                        "parser grammar {} has no rule {name}; a grammar transform can remove or inline a rule unless --entry-rule {name} keeps it",
                        self.report.grammar
                    ))
                })?,
            None => 0,
        };
        let rule_name = self.rule_names.get(index).ok_or_else(|| {
            EmitError::Config(format!(
                "parser grammar {} has no rules",
                self.report.grammar
            ))
        })?;
        Ok((index, rule_name.as_str()))
    }

    fn explain_request(&self, request: &ExplainRequest) -> Result<String, EmitError> {
        let resolved: Vec<(usize, &'c str)> = request
            .cases
            .iter()
            .map(|case| {
                self.resolve_start_rule(
                    case.start_rule.as_deref().or(request.start_rule.as_deref()),
                )
            })
            .collect::<Result<_, _>>()?;
        if let [case] = request.cases.as_slice() {
            let (start_rule, start_rule_name) = resolved[0];
            return Ok(self
                .explain_single(case, start_rule, start_rule_name, request.verbose)
                .report);
        }
        let total = request.cases.len();
        let mut entries = Vec::with_capacity(total);
        for (index, (case, (start_rule, start_rule_name))) in
            request.cases.iter().zip(resolved).enumerate()
        {
            entries.push(self.explain_batch_case(
                index + 1,
                total,
                case,
                start_rule,
                start_rule_name,
                request.verbose,
            ));
        }
        Ok(render::render_batch(
            &self.report.grammar,
            self.mode,
            &entries,
        ))
    }

    fn explain_single(
        &self,
        case: &ExplainCase,
        start_rule: usize,
        start_rule_name: &str,
        verbose: bool,
    ) -> Explained {
        let lexed = self.lexer_spec.tokenize(&case.input);
        if !lexed.errors.is_empty() {
            return Explained {
                report: render::render_lexer_errors(
                    &self.report.grammar,
                    &case.input_name,
                    &case.input,
                    &lexed.errors,
                ),
                traces: None,
            };
        }
        let (static_trace, ll_trace, verdict, probe) = self.simulate_case(&case.input, start_rule);
        let report = render::Explanation {
            start_rule: start_rule_name,
            mode: self.mode,
            input_name: &case.input_name,
            input: &case.input,
            lexed: &lexed,
            report: self.report,
            static_trace: &static_trace,
            ll_trace: &ll_trace,
            verdict,
            probe,
            verbose,
        }
        .render();
        Explained {
            report,
            traces: Some((static_trace, ll_trace, verdict)),
        }
    }

    fn explain_batch_case(
        &self,
        index: usize,
        total: usize,
        case: &ExplainCase,
        start_rule: usize,
        start_rule_name: &str,
        verbose: bool,
    ) -> render::BatchEntry {
        let lexed = self.lexer_spec.tokenize(&case.input);
        if !lexed.errors.is_empty() {
            return render::render_lexer_errors_in_batch(
                index,
                total,
                &case.input_name,
                &case.input,
                &lexed.errors,
            );
        }
        let (static_trace, ll_trace, verdict, probe) = self.simulate_case(&case.input, start_rule);
        render::Explanation {
            start_rule: start_rule_name,
            mode: self.mode,
            input_name: &case.input_name,
            input: &case.input,
            lexed: &lexed,
            report: self.report,
            static_trace: &static_trace,
            ll_trace: &ll_trace,
            verdict,
            probe,
            verbose,
        }
        .render_in_batch(index, total)
    }

    fn simulate_case(
        &self,
        input: &str,
        start_rule: usize,
    ) -> (
        interp::Trace,
        interp::Trace,
        diff::Verdict,
        Option<Option<i32>>,
    ) {
        let static_trace = interp::simulate(
            &self.spec,
            &self.lexer_spec,
            input,
            &self.static_oracle,
            self.static_manager,
            start_rule,
        );
        let ll_trace = interp::simulate(
            &self.spec,
            &self.lexer_spec,
            input,
            &Oracle::FullLl { probe: None },
            self.ll_manager,
            start_rule,
        );
        let verdict = diff::compare(&static_trace, &ll_trace);
        let probe = match verdict {
            diff::Verdict::Divergence { step } => {
                interp::simulate(
                    &self.spec,
                    &self.lexer_spec,
                    input,
                    &Oracle::FullLl { probe: Some(step) },
                    self.sll_manager,
                    start_rule,
                )
                .probe
            }
            _ => None,
        };
        (static_trace, ll_trace, verdict, probe)
    }
}

#[cfg(test)]
fn explain_units(
    compilation: &Compilation,
    parser: &CompiledParser,
    lexer: &CompiledLexer,
    fallback: AtnFallback,
    request: &ExplainRequest,
) -> Result<Explained, EmitError> {
    let session = ExplainSession::new(compilation, parser, lexer, fallback)?;
    let case = request
        .cases
        .first()
        .ok_or_else(|| EmitError::Config("--explain needs at least one input".to_owned()))?;
    let (start_rule, start_rule_name) =
        session.resolve_start_rule(case.start_rule.as_deref().or(request.start_rule.as_deref()))?;
    Ok(session.explain_single(case, start_rule, start_rule_name, request.verbose))
}

/// The `LL(1)` lookahead of each decision that the generated parser decides
/// with an inline token switch.
fn ll1_decisions(
    parser: &CompiledParser,
) -> HashMap<usize, &crate::grammar::atn::DecisionLookahead> {
    let by_state = parser
        .analysis
        .decision_lookahead
        .iter()
        .filter(|lookahead| lookahead.disjoint)
        .map(|lookahead| (lookahead.state, lookahead))
        .collect::<HashMap<_, _>>();
    parser
        .atn
        .decision_to_state
        .iter()
        .enumerate()
        .filter_map(|(decision, state)| {
            let state = usize::try_from(state.get_state_number()).ok()?;
            by_state.get(&state).map(|lookahead| (decision, *lookahead))
        })
        .collect()
}
