"""Syntax errors, error recovery, and parses that fail."""

from __future__ import annotations

import pytest

from dbt_antlr import GrammarBundle, NodeKind, ParseError, ParseSyntaxError

MISSING_PAREN = "{ ( a + 1 }"


def test_syntax_errors_raise_without_recovery(labels_parser) -> None:
    context = labels_parser.s(MISSING_PAREN)
    with pytest.raises(ParseSyntaxError) as error:
        with context:
            pytest.fail("the parse has a syntax error")
    assert isinstance(error.value, ParseError)
    assert str(error.value) == "line 1:10 missing ')' at '}'"
    (diagnostic,) = error.value.errors
    assert diagnostic.line == 1
    assert diagnostic.column == 10
    assert diagnostic.message == "missing ')' at '}'"
    assert diagnostic.origin == "parser"
    assert diagnostic.token_type == labels_parser.T__1
    assert (diagnostic.start_byte, diagnostic.stop_byte) == (10, 10)
    assert str(diagnostic) == "line 1:10 missing ')' at '}'"
    # The context frees the session at once and can parse again.
    assert context._session is None
    with pytest.raises(ParseSyntaxError):
        with context:
            pass


def test_recovery_keeps_error_nodes(labels_parser) -> None:
    with labels_parser.s(MISSING_PAREN, recovery=True) as tree:
        (diagnostic,) = tree.syntax_errors
        assert diagnostic.message == "missing ')' at '}'"
        assert tree.to_string_tree() == "(s { (blk (s (e ( (e (e a) + (e 1)) <missing ')'>))) })"
        parens = tree[1][0][0]
        assert parens.alt_label == "parens"
        missing = parens[-1]
        assert missing.kind is NodeKind.ERROR
        assert missing.is_error
        token = missing.symbol
        assert token.text == "<missing ')'>"
        assert (token.start_byte, token.stop_byte, token.token_index) == (-1, -1, -1)
        assert token.byte_span is None
        assert missing.byte_span is None
        assert missing.source_text == ""
        assert missing.get_text() == "<missing ')'>"
        assert (token.line, token.column) == (1, 10)
        assert parens.get_text() == "(a+1<missing ')'>"
        assert parens.source_text == "( a + 1"


def test_clean_parses_have_no_syntax_errors(labels_parser) -> None:
    with labels_parser.s("{ (a) }", recovery=True) as tree:
        assert tree.syntax_errors == ()


def test_extraneous_tokens_become_error_nodes(labels_parser) -> None:
    with labels_parser.s("{ 1 1 }", recovery=True) as tree:
        assert len(tree.syntax_errors) == 1
        assert tree.syntax_errors[0].message == "extraneous input '1' expecting '}'"
        errors = [leaf for leaf in tree.children if leaf.is_error]
        assert [leaf.get_text() for leaf in errors] == ["1"]
        assert errors[0].source_text == "1"
        assert errors[0].symbol.byte_span == (4, 5)


def test_lexer_errors(csv_parser) -> None:
    with csv_parser.csvFile('a\n"open\n', recovery=True) as tree:
        lexer_errors = [error for error in tree.syntax_errors if error.origin == "lexer"]
        assert lexer_errors
        error = lexer_errors[0]
        assert error.message.startswith("token recognition error at: ")
        assert (error.line, error.column, error.start_byte) == (2, 0, 2)
        assert error.token_type is None


def test_panics_raise_parse_error(keywords_parser) -> None:
    with pytest.raises(ParseError, match="the parser panicked: boom"):
        with keywords_parser.from_("class a boom"):
            pass
    with keywords_parser.from_("class a") as tree:
        assert tree.get_text() == "classa<EOF>"


def test_arena_allocation_limit(native, csv_parser) -> None:
    bundle = GrammarBundle.load(native)
    with pytest.raises(ValueError):
        bundle.set_arena_allocation_limit(-1)
    bundle.set_arena_allocation_limit(1024)
    try:
        with pytest.raises(ParseError, match="limit"):
            with csv_parser.csvFile("a,b,c\n" * 100):
                pass
    finally:
        bundle.set_arena_allocation_limit(0)
    with csv_parser.csvFile("a,b,c\n" * 100) as tree:
        assert len(tree) == 100


def test_python_keywords_and_entry_rules(keywords_parser) -> None:
    for name in ("from_", "statement", "class_", "lambda_", "boom"):
        assert callable(getattr(keywords_parser, name))
    assert not hasattr(keywords_parser, "nested")
    assert keywords_parser.RULE_nested == 4
    with keywords_parser.lambda_("lambda x") as tree:
        assert tree.to_string_tree() == "(lambda lambda (nested x))"
        assert tree[1].rule_name == "nested"
    grammar = keywords_parser._grammar
    assert grammar.entry_rules == (True, True, True, True, False, True)
    with pytest.raises(ValueError, match="arguments"):
        grammar.parse(keywords_parser.RULE_nested, "x")
    with pytest.raises(ValueError, match="no rule"):
        grammar.parse(99, "x")


def test_grammar_lookup_errors(native) -> None:
    bundle = GrammarBundle.load(native)
    assert GrammarBundle.load(native) is bundle
    with pytest.raises(ImportError, match="exports no grammar"):
        bundle.grammar("CSV")
    with pytest.raises(ImportError, match="do not match"):
        bundle.grammar("csv", rule_names=("csvFile",))
    grammar = bundle.grammar("csv")
    assert grammar.rule_names == ("csvFile", "hdr", "row", "field")
    assert grammar.rule_index("row") == 2
    assert grammar.token_name(5) == "TEXT"
    assert grammar.token_name(1) == "','"
    assert grammar.token_name(-1) == "EOF"
    assert grammar.token_name(99) is None


def test_abi_version_handshake(native) -> None:
    import types

    assert native.lib.antlr_abi_version() == 1
    stale_lib = types.SimpleNamespace(antlr_abi_version=lambda: 999)
    stale_native = types.SimpleNamespace(ffi=native.ffi, lib=stale_lib)
    with pytest.raises(RuntimeError, match="ABI version mismatch"):
        GrammarBundle.load(stale_native)

    missing_native = types.SimpleNamespace(ffi=native.ffi, lib=types.SimpleNamespace())
    with pytest.raises(RuntimeError, match="antlr_abi_version"):
        GrammarBundle.load(missing_native)
