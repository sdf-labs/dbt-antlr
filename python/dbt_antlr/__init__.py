"""Python bindings of dbt-antlr parse trees.

A grammar bundle is a native extension module that links the Rust parsers
of one or more grammars. The ``<name>_parser.py`` module that
``dbt-antlr-codegen --python`` generates for a grammar has one entry
function per parser rule::

    import my_bundle.csv_parser as csv_parser

    with csv_parser.csvFile("a,b\\n") as tree:
        print(tree.to_string_tree())

``dbt_antlr.build`` builds the native extension module with CFFI.
"""

from ._runtime import (
    Grammar,
    GrammarBundle,
    NodeKind,
    ParseContext,
    ParseError,
    ParseSyntaxError,
    SyntaxDiagnostic,
    Token,
    Tree,
)

for _public in (
    Grammar,
    GrammarBundle,
    NodeKind,
    ParseContext,
    ParseError,
    ParseSyntaxError,
    SyntaxDiagnostic,
    Token,
    Tree,
):
    _public.__module__ = __name__
del _public

__all__ = [
    "Grammar",
    "GrammarBundle",
    "NodeKind",
    "ParseContext",
    "ParseError",
    "ParseSyntaxError",
    "SyntaxDiagnostic",
    "Token",
    "Tree",
]
