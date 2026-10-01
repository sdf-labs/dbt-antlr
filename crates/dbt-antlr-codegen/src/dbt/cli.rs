// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Command-line interface of the dbt emission layer, mirroring the Java
//! ANTLR tool's argument syntax:
//!
//! ```text
//! dbt-antlr [OPTIONS] <grammar.g4>
//!   -o <dir>          output directory (default: current dir)
//!   -lib <dir>        grammar/library search dir for imports (repeatable)
//!   -visitor          generate visitor (+base visitor)
//!   -no-listener      do not generate listener (+base listener)
//!   -Dkey=value       ignored with a warning (compat)
//! ```

use std::io::Write;
use std::path::PathBuf;

use miette::{Context as _, IntoDiagnostic as _};

use crate::dbt::emit_files_with_flags;

const USAGE: &str = "\
usage: dbt-antlr [OPTIONS] <grammar.g4>
  -o <dir>          output directory (default: current dir)
  -lib <dir>        grammar/library search dir for imports (repeatable)
  -visitor          generate visitor (+base visitor)
  -no-listener      do not generate listener (+base listener)
  -Dkey=value       ignored with a warning (compat)
";

/// Parsed command line.
#[derive(Debug)]
struct CliArgs {
    grammar: PathBuf,
    out_dir: PathBuf,
    lib_dirs: Vec<PathBuf>,
    gen_listener: bool,
    gen_visitor: bool,
}

fn parse_args(args: &[String], stderr: &mut impl Write) -> miette::Result<CliArgs> {
    let mut grammar = None;
    let mut out_dir = PathBuf::from(".");
    let mut lib_dirs = Vec::new();
    let mut gen_listener = true;
    let mut gen_visitor = false;

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
            "-h" | "-help" | "--help" => miette::bail!("{USAGE}"),
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
    Ok(CliArgs {
        grammar,
        out_dir,
        lib_dirs,
        gen_listener,
        gen_visitor,
    })
}

fn value_after(option: &str, args: &[String], index: &mut usize) -> miette::Result<String> {
    args.get(*index).map_or_else(
        || miette::bail!("option {option} expects a directory\n{USAGE}"),
        |value| {
            *index += 1;
            Ok(value.clone())
        },
    )
}

/// Runs the command line against the process arguments and streams.
pub fn run_cli() -> miette::Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut stdout = std::io::stdout().lock();
    let mut stderr = std::io::stderr().lock();
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
    let files = emit_files_with_flags(
        &cli.grammar,
        &cli.lib_dirs,
        cli.gen_listener,
        cli.gen_visitor,
    )
    .into_diagnostic()
    .wrap_err_with(|| format!("cannot emit {}", cli.grammar.display()))?;
    std::fs::create_dir_all(&cli.out_dir)
        .into_diagnostic()
        .wrap_err_with(|| format!("cannot create {}", cli.out_dir.display()))?;
    for file in &files {
        let path = cli.out_dir.join(&file.name);
        std::fs::write(&path, &file.content)
            .into_diagnostic()
            .wrap_err_with(|| format!("cannot write {}", path.display()))?;
        writeln!(stdout, "{}", file.name).into_diagnostic()?;
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
    fn missing_grammar_is_an_error() {
        assert!(parse(&["-visitor"]).is_err());
    }

    #[test]
    fn missing_option_value_is_an_error() {
        assert!(parse(&["-o"]).is_err());
        assert!(parse(&["T.g4", "-lib"]).is_err());
    }
}
