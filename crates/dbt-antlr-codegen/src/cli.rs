// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Command-line interface of the dbt emission layer, mirroring the Java
//! ANTLR tool's argument syntax:
//!
//! ```text
//! dbt-antlr-codegen [OPTIONS] <grammar.g4>
//!   -o <dir>          output directory (default: current dir)
//!   -lib <dir>        grammar/library search dir for imports (repeatable)
//!   -visitor          generate visitor (+base visitor)
//!   -no-listener      do not generate listener (+base listener)
//!   -Dkey=value       ignored with a warning (compat)
//!   -n, --dry-run, --report-only  compile, analyze, and optimize without writing output files
//!   --static-lexer-dfa            precompute static lexer DFA tables (lexer scope only)
//!   --static-parser-dfa           precompute static parser DFA tables (parser scope only)
//!   --static-dfa-only             generate only fully static lexer and parser DFA tables
//!                                 (implies --static-lexer-dfa and --static-parser-dfa)
//!   --report-decisions            print the parser decision report (implies report-only)
//!   --dump-dfas <SPEC>            print the static parser DFA tables matching SPEC
//!                                 (implies report-only)
//!   --profile-decisions           print the time and work of each parser decision
//!                                 analysis (implies report-only)
//!   --entry-rule <NAME>           mark NAME as a parser entry rule (repeatable)
//!   --prune-unreachable-rules     delete parser rules unreachable from the entry rules
//!   --inline-trivial-rules        inline trivial pure parser rules into their callers
//!   --optimize-precedence-ladders collapse precedence ladders into left-recursive rules
//!   --explain <INPUT>             explain how the static DFA parser parses INPUT, compared
//!                                 with full LL (needs --static-dfa-only or
//!                                 --static-parser-dfa; implies report-only)
//!   --explain-file <PATH>         like --explain, with the input in PATH (- reads stdin)
//!   --start-rule <NAME>           the parser rule that --explain starts from
//!                                 (default: the first rule)
//!   --lexer <Lexer.g4>            the lexer grammar that tokenizes the --explain input
//! ```

use std::io::Write;
use std::path::PathBuf;

use miette::{Context as _, IntoDiagnostic as _};

const USAGE: &str = "\
usage: dbt-antlr-codegen [OPTIONS] <grammar.g4>
  -o <dir>          output directory (default: current dir)
  -lib <dir>        grammar/library search dir for imports (repeatable)
  -visitor          generate visitor (+base visitor)
  -no-listener      do not generate listener (+base listener)
  -Dkey=value       ignored with a warning (compat)
  -n, --dry-run, --report-only  compile, analyze, and optimize without writing output files
  --static-lexer-dfa            precompute static lexer DFA tables (lexer scope only)
  --static-parser-dfa           precompute static parser DFA tables (parser scope only)
  --static-dfa-only             generate only fully static lexer and parser DFA tables
                                (implies --static-lexer-dfa and --static-parser-dfa)
  --report-decisions            print the parser decision report (implies report-only)
  --dump-dfas <SPEC>            print the static parser DFA tables matching SPEC
                                (implies report-only)
  --profile-decisions           print the time and work of each parser decision
                                analysis (implies report-only)
  --entry-rule <NAME>           mark NAME as a parser entry rule (repeatable)
  --prune-unreachable-rules     delete parser rules unreachable from the entry rules
  --inline-trivial-rules        inline trivial pure parser rules into their callers
  --optimize-precedence-ladders collapse precedence ladders into left-recursive rules
  --explain <INPUT>             explain how the static DFA parser parses INPUT, compared
                                with full LL (needs --static-dfa-only or
                                --static-parser-dfa; implies report-only)
  --explain-file <PATH>         like --explain, with the input in PATH (- reads stdin)
  --start-rule <NAME>           the parser rule that --explain starts from
                                (default: the first rule)
  --lexer <Lexer.g4>            the lexer grammar that tokenizes the --explain input
";

/// Parsed command line.
#[derive(Debug)]
struct CliArgs {
    grammar: PathBuf,
    out_dir: PathBuf,
    lib_dirs: Vec<PathBuf>,
    gen_listener: bool,
    gen_visitor: bool,
    static_lexer_dfa: bool,
    static_parser_dfa: bool,
    static_dfa_only: bool,
    report_decisions: bool,
    dump_dfas: Option<String>,
    profile_decisions: bool,
    entry_rules: Vec<String>,
    prune_unreachable: bool,
    inline_trivial: bool,
    optimize_ladders: bool,
    report_only: bool,
    explain: Option<ExplainInput>,
    start_rule: Option<String>,
    lexer: Option<PathBuf>,
}

/// Where the `--explain` input comes from.
#[derive(Debug, Eq, PartialEq)]
enum ExplainInput {
    Text(String),
    File(PathBuf),
}

fn parse_args(args: &[String], stderr: &mut impl Write) -> miette::Result<CliArgs> {
    let mut grammar = None;
    let mut out_dir = PathBuf::from(".");
    let mut lib_dirs = Vec::new();
    let mut gen_listener = true;
    let mut gen_visitor = false;
    let mut static_lexer_dfa = false;
    let mut static_parser_dfa = false;
    let mut static_dfa_only = false;
    let mut report_decisions = false;
    let mut dump_dfas = None;
    let mut profile_decisions = false;
    let mut entry_rules = Vec::new();
    let mut prune_unreachable = false;
    let mut inline_trivial = false;
    let mut optimize_ladders = false;
    let mut report_only = false;
    let mut explain = None;
    let mut start_rule = None;
    let mut lexer = None;

    let mut index = 0;
    while index < args.len() {
        let arg = &args[index];
        index += 1;
        match arg.as_str() {
            "-o" => {
                out_dir = PathBuf::from(value_after(arg, args, &mut index)?);
            }
            "-lib" => {
                lib_dirs.push(PathBuf::from(value_after(arg, args, &mut index)?));
            }
            "-visitor" => gen_visitor = true,
            "-no-visitor" => gen_visitor = false,
            "-listener" => gen_listener = true,
            "-no-listener" => gen_listener = false,
            "-n" | "--dry-run" | "--report-only" => report_only = true,
            "--static-lexer-dfa" => static_lexer_dfa = true,
            "--static-parser-dfa" => static_parser_dfa = true,
            "--static-dfa-only" => static_dfa_only = true,
            "--report-decisions" => report_decisions = true,
            "--profile-decisions" => profile_decisions = true,
            "--dump-dfas" => {
                dump_dfas = Some(value_after(arg, args, &mut index)?);
            }
            _ if arg.starts_with("--dump-dfas=") => {
                let spec = arg.strip_prefix("--dump-dfas=").unwrap_or_default();
                if spec.is_empty() {
                    miette::bail!("option --dump-dfas expects a value\n{USAGE}");
                }
                dump_dfas = Some(spec.to_owned());
            }
            "--entry-rule" => {
                entry_rules.push(value_after(arg, args, &mut index)?);
            }
            "--prune-unreachable-rules" => prune_unreachable = true,
            "--inline-trivial-rules" => inline_trivial = true,
            "--optimize-precedence-ladders" => optimize_ladders = true,
            "--explain" | "--explain-file" => {
                let value = value_after(arg, args, &mut index)?;
                let input = if arg == "--explain" {
                    ExplainInput::Text(value)
                } else {
                    ExplainInput::File(PathBuf::from(value))
                };
                if explain.replace(input).is_some() {
                    miette::bail!("expected at most one --explain or --explain-file\n{USAGE}");
                }
            }
            "--start-rule" => {
                if start_rule
                    .replace(value_after(arg, args, &mut index)?)
                    .is_some()
                {
                    miette::bail!("expected at most one --start-rule\n{USAGE}");
                }
            }
            "--lexer" => {
                let path = PathBuf::from(value_after(arg, args, &mut index)?);
                if lexer.replace(path).is_some() {
                    miette::bail!("expected at most one --lexer\n{USAGE}");
                }
            }
            "-h" | "-help" | "--help" => return Err(miette::miette!("{USAGE}")),
            _ if arg.starts_with("-D") => {
                writeln!(stderr, "warning: ignoring option {arg}").into_diagnostic()?;
            }
            _ if arg.starts_with('-') && arg.len() > 1 => {
                miette::bail!("unknown option {arg}\n{USAGE}");
            }
            _ => {
                if grammar.replace(PathBuf::from(arg)).is_some() {
                    miette::bail!("expected exactly one grammar file\n{USAGE}");
                }
            }
        }
    }

    let Some(grammar) = grammar else {
        miette::bail!("missing grammar file\n{USAGE}");
    };
    if explain.is_none() && (start_rule.is_some() || lexer.is_some()) {
        miette::bail!("--start-rule and --lexer need --explain or --explain-file\n{USAGE}");
    }
    Ok(CliArgs {
        grammar,
        out_dir,
        lib_dirs,
        gen_listener,
        gen_visitor,
        static_lexer_dfa,
        static_parser_dfa,
        static_dfa_only,
        report_decisions,
        dump_dfas,
        profile_decisions,
        entry_rules,
        prune_unreachable,
        inline_trivial,
        optimize_ladders,
        report_only,
        explain,
        start_rule,
        lexer,
    })
}

fn value_after(option: &str, args: &[String], index: &mut usize) -> miette::Result<String> {
    args.get(*index).map_or_else(
        || Err(miette::miette!("option {option} expects a value\n{USAGE}")),
        |value| {
            *index += 1;
            Ok(value.clone())
        },
    )
}

/// Renders one non-fatal diagnostic in the `warning[G4S078]: path:line:
/// column: message` format the Java tool's Rust target diagnostics use.
fn render_diagnostic(diagnostic: &crate::Diagnostic) -> String {
    let severity = match diagnostic.severity() {
        crate::Severity::Warning => "warning",
        crate::Severity::Error => "error",
    };
    let position = match (diagnostic.line(), diagnostic.column()) {
        (Some(line), Some(column)) => format!(":{line}:{column}"),
        _ => String::new(),
    };
    format!(
        "{severity}[{}]: {}{position}: {}",
        diagnostic.code(),
        diagnostic.path().display(),
        diagnostic.message()
    )
}

/// Runs the command line against the process arguments and streams.
pub fn run_cli() -> miette::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    run(&args, &mut stdout, &mut stderr)
}

/// Runs the command line with explicit arguments and output streams.
///
/// Emits every file of [`emit_files_with_flags`] (visitor default off,
/// listener default on, matching the Java tool) into the output directory
/// and prints each written file name to `stdout`.
pub fn run(
    args: &[String],
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> miette::Result<()> {
    let cli = parse_args(args, stderr)?;
    if cli.explain.is_some() {
        return run_explain(&cli, stdout, stderr);
    }
    let mut config = crate::Config::new(&cli.grammar);
    config
        .out_dir(&cli.out_dir)
        .listener(cli.gen_listener)
        .visitor(cli.gen_visitor)
        .cargo_directives(false)
        .static_lexer_dfa(cli.static_lexer_dfa)
        .static_parser_dfa(cli.static_parser_dfa)
        .static_dfa_only(cli.static_dfa_only)
        .report_decisions(cli.report_decisions)
        .profile_decisions(cli.profile_decisions)
        .prune_unreachable_rules(cli.prune_unreachable)
        .inline_trivial_rules(cli.inline_trivial)
        .optimize_precedence_ladders(cli.optimize_ladders)
        .report_only(cli.report_only);
    if let Some(spec) = &cli.dump_dfas {
        config.dump_dfas(spec);
    }
    for dir in &cli.lib_dirs {
        config.lib_dir(dir);
    }
    for entry in &cli.entry_rules {
        config.entry_rule(entry);
    }
    let generation = config
        .generate_detailed()
        .into_diagnostic()
        .wrap_err_with(|| format!("cannot emit {}", cli.grammar.display()))?;
    let mut errors = 0;
    for diagnostic in generation.diagnostics() {
        if diagnostic.severity() == crate::Severity::Error {
            errors += 1;
        }
        writeln!(stderr, "{}", render_diagnostic(diagnostic)).into_diagnostic()?;
    }
    for message in generation.optimization_messages() {
        writeln!(stderr, "{message}").into_diagnostic()?;
    }
    for report in generation.reports() {
        write!(stdout, "{report}").into_diagnostic()?;
    }
    for path in generation.written_files() {
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );
        writeln!(stdout, "{name}").into_diagnostic()?;
    }
    if errors > 0 {
        miette::bail!("failed with {errors} error(s)");
    }
    Ok(())
}
/// Runs `--explain` / `--explain-file`.
fn run_explain(
    cli: &CliArgs,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> miette::Result<()> {
    let (input, input_name) = match &cli.explain {
        Some(ExplainInput::Text(text)) => (text.clone(), "<inline>".to_owned()),
        Some(ExplainInput::File(path)) if path.as_os_str() == "-" => {
            let mut text = String::new();
            std::io::Read::read_to_string(&mut std::io::stdin(), &mut text)
                .into_diagnostic()
                .wrap_err("cannot read the --explain-file input from stdin")?;
            (text, "<stdin>".to_owned())
        }
        Some(ExplainInput::File(path)) => (
            std::fs::read_to_string(path)
                .into_diagnostic()
                .wrap_err_with(|| format!("cannot read --explain-file {}", path.display()))?,
            path.display().to_string(),
        ),
        None => unreachable!("run_explain runs only with an --explain input"),
    };
    let mut options = crate::OptimizationOptions::new();
    options
        .static_lexer_dfa(cli.static_lexer_dfa)
        .static_parser_dfa(cli.static_parser_dfa)
        .static_dfa_only(cli.static_dfa_only)
        .report_decisions(cli.report_decisions)
        .profile_decisions(cli.profile_decisions)
        .prune_unreachable_rules(cli.prune_unreachable)
        .inline_trivial_rules(cli.inline_trivial)
        .optimize_precedence_ladders(cli.optimize_ladders);
    if let Some(spec) = &cli.dump_dfas {
        options.dump_dfas(spec);
    }
    for entry in &cli.entry_rules {
        options.entry_rule(entry);
    }
    let request = crate::explain::ExplainRequest {
        input,
        input_name,
        start_rule: cli.start_rule.clone(),
        lexer: cli.lexer.clone(),
    };
    let output = crate::explain::explain(&cli.grammar, &cli.lib_dirs, &options, &request)
        .into_diagnostic()
        .wrap_err_with(|| format!("cannot explain {}", cli.grammar.display()))?;
    let mut errors = 0;
    for diagnostic in &output.diagnostics {
        if diagnostic.severity() == crate::Severity::Error {
            errors += 1;
        }
        writeln!(stderr, "{}", render_diagnostic(diagnostic)).into_diagnostic()?;
    }
    for message in &output.optimization_messages {
        writeln!(stderr, "{message}").into_diagnostic()?;
    }
    for report in &output.reports {
        write!(stdout, "{report}").into_diagnostic()?;
    }
    if errors > 0 {
        miette::bail!("failed with {errors} error(s)");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_args;

    fn parse(args: &[&str]) -> miette::Result<super::CliArgs> {
        let mut stderr = Vec::new();
        parse_args(
            &args.iter().map(ToString::to_string).collect::<Vec<_>>(),
            &mut stderr,
        )
    }

    #[test]
    fn defaults_match_the_java_tool() {
        let cli = parse(&["T.g4"]).expect("minimal command line parses");
        assert_eq!(cli.grammar, std::path::PathBuf::from("T.g4"));
        assert_eq!(cli.out_dir, std::path::PathBuf::from("."));
        assert!(cli.gen_listener);
        assert!(!cli.gen_visitor);
    }

    #[test]
    fn static_lexer_dfa_flag_parses() {
        let cli = parse(&["--static-lexer-dfa", "T.g4"]).expect("flag parses");
        assert!(cli.static_lexer_dfa);
        let cli = parse(&["T.g4"]).expect("minimal command line parses");
        assert!(!cli.static_lexer_dfa);
    }

    #[test]
    fn static_parser_dfa_and_report_flags_parse() {
        let cli = parse(&["--static-parser-dfa", "T.g4"]).expect("flag parses");
        assert!(cli.static_parser_dfa);
        assert!(!cli.report_decisions);
        assert_eq!(cli.dump_dfas, None);
        assert!(!cli.profile_decisions);

        let cli = parse(&[
            "--report-decisions",
            "--dump-dfas=all",
            "--profile-decisions",
            "T.g4",
        ])
        .expect("report flags parse");
        assert!(cli.report_decisions);
        assert!(cli.profile_decisions);
        assert_eq!(cli.dump_dfas.as_deref(), Some("all"));

        let cli = parse(&["--dump-dfas", "paths,row:2", "T.g4"]).expect("spaced value parses");
        assert_eq!(cli.dump_dfas.as_deref(), Some("paths,row:2"));

        assert!(parse(&["--dump-dfas=", "T.g4"]).is_err());
        assert!(parse(&["T.g4", "--dump-dfas"]).is_err());
    }

    #[test]
    fn static_dfa_only_flag_parses() {
        let cli = parse(&["--static-dfa-only", "T.g4"]).expect("flag parses");
        assert!(cli.static_dfa_only);
        let cli = parse(&["T.g4"]).expect("minimal command line parses");
        assert!(!cli.static_dfa_only);
    }

    #[test]
    fn report_only_and_aliases_parse() {
        for flag in ["-n", "--dry-run", "--report-only"] {
            let cli = parse(&[flag, "--static-lexer-dfa", "T.g4"]).expect("flag parses");
            assert!(cli.report_only, "{flag} sets report_only");
            assert!(cli.static_lexer_dfa);
        }
    }

    #[test]
    fn repeatable_lib_dirs() {
        let cli =
            parse(&["-lib", "a", "-lib", "b", "-visitor", "T.g4"]).expect("repeated -lib parses");
        assert_eq!(
            cli.lib_dirs,
            vec![std::path::PathBuf::from("a"), std::path::PathBuf::from("b")]
        );
        assert!(cli.gen_visitor);
    }

    #[test]
    fn define_options_are_ignored_with_a_warning() {
        let mut stderr = Vec::new();
        let args = vec!["-Dlanguage=Rust".to_owned(), "T.g4".to_owned()];
        let cli = parse_args(&args, &mut stderr).expect("-D option parses");
        assert_eq!(cli.grammar, std::path::PathBuf::from("T.g4"));
        let warning = String::from_utf8(stderr).expect("warning is utf-8");
        assert!(warning.contains("ignoring option -Dlanguage=Rust"));
    }

    #[test]
    fn explain_flags_parse() {
        let cli = parse(&[
            "--static-dfa-only",
            "--explain",
            "a b",
            "--start-rule",
            "prog",
            "--lexer",
            "L.g4",
            "T.g4",
        ])
        .expect("explain flags parse");
        assert_eq!(
            cli.explain,
            Some(super::ExplainInput::Text("a b".to_owned()))
        );
        assert_eq!(cli.start_rule.as_deref(), Some("prog"));
        assert_eq!(cli.lexer, Some(std::path::PathBuf::from("L.g4")));

        let cli = parse(&["--explain-file", "-", "T.g4"]).expect("explain file parses");
        assert_eq!(
            cli.explain,
            Some(super::ExplainInput::File(std::path::PathBuf::from("-")))
        );
    }

    #[test]
    fn explain_flags_are_checked() {
        assert!(parse(&["--explain", "a", "--explain-file", "f", "T.g4"]).is_err());
        assert!(
            parse(&[
                "--explain",
                "a",
                "--start-rule",
                "x",
                "--start-rule",
                "y",
                "T.g4"
            ])
            .is_err()
        );
        assert!(
            parse(&[
                "--explain",
                "a",
                "--lexer",
                "A.g4",
                "--lexer",
                "B.g4",
                "T.g4"
            ])
            .is_err()
        );
        assert!(parse(&["--start-rule", "x", "T.g4"]).is_err());
        assert!(parse(&["--lexer", "L.g4", "T.g4"]).is_err());
        assert!(parse(&["T.g4", "--explain"]).is_err());
    }

    #[test]
    fn missing_grammar_is_an_error() {
        assert!(parse(&["-visitor"]).is_err());
    }

    #[test]
    fn missing_option_value_is_an_error() {
        assert!(parse(&["-o"]).is_err());
        assert!(parse(&["T.g4", "-lib"]).is_err());
    }
}
