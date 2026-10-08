// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Conversion of a front-end [`CompilationError`] into the crate's public
//! [`Error`] type. Shared by the dbt emission layer and the ATN export
//! entry point.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use crate::error::{Diagnostic, Error, Severity};
use crate::grammar::{self, frontend::SourceSpan};

pub(crate) fn compilation_error(
    error: &grammar::diagnostic::CompilationError,
    roots: &[PathBuf],
) -> Error {
    let fallback = roots
        .first()
        .map_or_else(|| Path::new("<grammar>"), PathBuf::as_path);
    let mut message = String::new();
    let mut diagnostics = Vec::with_capacity(error.diagnostics().len());
    for (index, diagnostic) in error.diagnostics().iter().enumerate() {
        let (severity_name, severity) = match diagnostic.severity {
            grammar::diagnostic::Severity::Warning => ("warning", Severity::Warning),
            grammar::diagnostic::Severity::Error => ("error", Severity::Error),
        };
        let location = error.location(index);
        let path =
            location.map_or_else(|| fallback.to_path_buf(), |location| location.path.clone());
        let primary = diagnostic.primary_source_span();
        let position = location.and_then(|location| location.position);
        let structured_position = primary.and(position);
        let byte_span = location.and(primary).map(source_byte_span);
        let display_position =
            position.map_or_else(String::new, |(line, column)| format!(":{line}:{column}"));
        let _ = writeln!(
            message,
            "{severity_name}[{}]: {}{display_position}: {}",
            diagnostic.code,
            path.display(),
            diagnostic.message
        );
        diagnostics.push(Diagnostic::new(
            diagnostic.code,
            severity,
            diagnostic.message.clone(),
            path,
            structured_position,
            byte_span,
        ));
    }
    Error::compilation(message, diagnostics)
}

fn source_byte_span(span: &SourceSpan) -> std::ops::Range<usize> {
    usize::try_from(span.bytes.start).expect("source offset exceeds usize")
        ..usize::try_from(span.bytes.end).expect("source offset exceeds usize")
}
