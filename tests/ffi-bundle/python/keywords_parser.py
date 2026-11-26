# Generated from KeywordsParser.g4 by ANTLR 4.13.2
"""The entry functions of the ``KeywordsParser`` grammar.

Each entry function parses an input from one rule and returns a
``dbt_antlr.ParseContext``. Enter the context in a ``with`` statement to get
the parse tree. The tree is valid only in the ``with`` block.
"""

from __future__ import annotations

from dbt_antlr import GrammarBundle as _GrammarBundle
from dbt_antlr import ParseContext as _ParseContext

from . import _native

_grammar = _GrammarBundle.load(_native).grammar(
    "keywords",
    rule_names=(
        "from",
        "statement",
        "class",
        "lambda",
        "nested",
        "boom",
    ),
)

EOF = -1
CLASS = 1
LAMBDA = 2
BOOM = 3
ID = 4
WS = 5
END = 6

RULE_from = 0
RULE_statement = 1
RULE_class = 2
RULE_lambda = 3
RULE_nested = 4
RULE_boom = 5


def from_(input: str | bytes, *, recovery: bool = False) -> _ParseContext:
    """Parses ``input`` from the rule ``from``."""
    return _grammar.parse(0, input, recovery=recovery)


def statement(input: str | bytes, *, recovery: bool = False) -> _ParseContext:
    """Parses ``input`` from the rule ``statement``."""
    return _grammar.parse(1, input, recovery=recovery)


def class_(input: str | bytes, *, recovery: bool = False) -> _ParseContext:
    """Parses ``input`` from the rule ``class``."""
    return _grammar.parse(2, input, recovery=recovery)


def lambda_(input: str | bytes, *, recovery: bool = False) -> _ParseContext:
    """Parses ``input`` from the rule ``lambda``."""
    return _grammar.parse(3, input, recovery=recovery)


def boom(input: str | bytes, *, recovery: bool = False) -> _ParseContext:
    """Parses ``input`` from the rule ``boom``."""
    return _grammar.parse(5, input, recovery=recovery)
