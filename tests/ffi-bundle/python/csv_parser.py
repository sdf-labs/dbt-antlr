# Generated from CSV.g4 by ANTLR 4.13.2
"""The entry functions of the ``CSV`` grammar.

Each entry function parses an input from one rule and returns a
``dbt_antlr.ParseContext``. Enter the context in a ``with`` statement to get
the parse tree. The tree is valid only in the ``with`` block.
"""

from __future__ import annotations

from dbt_antlr import GrammarBundle as _GrammarBundle
from dbt_antlr import ParseContext as _ParseContext

from . import _native

_grammar = _GrammarBundle.load(_native).grammar(
    "csv",
    rule_names=(
        "csvFile",
        "hdr",
        "row",
        "field",
    ),
)

EOF = -1
T__0 = 1
T__1 = 2
T__2 = 3
WS = 4
TEXT = 5
STRING = 6

RULE_csvFile = 0
RULE_hdr = 1
RULE_row = 2
RULE_field = 3


def csvFile(input: str | bytes, *, recovery: bool = False) -> _ParseContext:
    """Parses ``input`` from the rule ``csvFile``."""
    return _grammar.parse(0, input, recovery=recovery)


def hdr(input: str | bytes, *, recovery: bool = False) -> _ParseContext:
    """Parses ``input`` from the rule ``hdr``."""
    return _grammar.parse(1, input, recovery=recovery)


def row(input: str | bytes, *, recovery: bool = False) -> _ParseContext:
    """Parses ``input`` from the rule ``row``."""
    return _grammar.parse(2, input, recovery=recovery)


def field(input: str | bytes, *, recovery: bool = False) -> _ParseContext:
    """Parses ``input`` from the rule ``field``."""
    return _grammar.parse(3, input, recovery=recovery)
