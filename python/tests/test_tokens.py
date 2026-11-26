"""Tokens: text, byte offsets, and locations."""

from __future__ import annotations

import pytest


def _leaves(tree):
    if not tree.is_rule:
        yield tree
        return
    for child in tree:
        yield from _leaves(child)


def _tokens(tree):
    return [leaf.symbol for leaf in _leaves(tree)]


def test_token_fields(csv_parser) -> None:
    with csv_parser.csvFile("ab,cd\nx\n") as tree:
        ab, comma, cd, newline, *_ = _tokens(tree)
        assert (ab.type, ab.channel, ab.token_index) == (csv_parser.TEXT, 0, 0)
        assert (ab.start_byte, ab.stop_byte, ab.byte_span) == (0, 1, (0, 2))
        assert (comma.type, comma.text) == (csv_parser.T__0, ",")
        assert cd.token_index == 2
        assert newline.text == "\n"
        assert repr(ab) == "<Token TEXT 'ab'>"


@pytest.mark.parametrize("as_bytes", [False, True])
def test_ascii_input(csv_parser, as_bytes: bool) -> None:
    text = "x,yy\nzzz\n"
    with csv_parser.csvFile(text.encode() if as_bytes else text) as tree:
        assert [token.text for token in _tokens(tree)] == ["x", ",", "yy", "\n", "zzz", "\n"]
        assert tree[1].source_text == "zzz\n"


@pytest.mark.parametrize("as_bytes", [False, True])
def test_multibyte_input(csv_parser, as_bytes: bool) -> None:
    # "é" has 2 UTF-8 bytes, "€" has 3, and "𝄞" has 4.
    text = "é,€𝄞\n\"ü\",b\n"
    with csv_parser.csvFile(text.encode() if as_bytes else text) as tree:
        tokens = _tokens(tree)
        assert [token.text for token in tokens] == ["é", ",", "€𝄞", "\n", '"ü"', ",", "b", "\n"]
        assert [token.byte_span for token in tokens] == [
            (0, 2), (2, 3), (3, 10), (10, 11), (11, 15), (15, 16), (16, 17), (17, 18),
        ]
        assert [(token.start_byte, token.stop_byte) for token in tokens[:3]] == [
            (0, 1), (2, 2), (3, 9),
        ]
        assert [(token.line, token.column) for token in tokens] == [
            (1, 0), (1, 1), (1, 2), (1, 4), (2, 0), (2, 3), (2, 4), (2, 5),
        ]
        assert tree[0].source_text == "é,€𝄞\n"
        assert tree[1].byte_span == (11, 18)
        assert tree.get_text() == text


def test_bytes_like_inputs(csv_parser) -> None:
    for data in (bytearray(b"a\nb\n"), memoryview(b"a\nb\n")):
        with csv_parser.csvFile(data) as tree:
            assert tree.get_text() == "a\nb\n"


def test_invalid_utf8_raises_unicode_decode_error(csv_parser) -> None:
    with pytest.raises(UnicodeDecodeError) as error:
        with csv_parser.csvFile(b"ok\n\xff\n"):
            pass
    assert error.value.start == 3


def test_lone_surrogate_raises_unicode_encode_error(csv_parser) -> None:
    with pytest.raises(UnicodeEncodeError):
        with csv_parser.csvFile("a\ud800\n"):
            pass


def test_non_text_input_raises_type_error(csv_parser) -> None:
    with pytest.raises(TypeError):
        csv_parser.csvFile(42)


def test_token_identity(csv_parser) -> None:
    with csv_parser.csvFile("a,b\nc\n") as tree:
        row = tree[0][0]
        assert row[1].symbol == row.token_by_type(csv_parser.T__0)
        assert row[1].symbol != row[3].symbol
        assert len({row[1].symbol, row.token_by_type(csv_parser.T__0)}) == 1
        assert row.start_token == row[0][0].symbol


def test_tokens_with_the_eof_type_have_the_eof_text(keywords_parser) -> None:
    # The lexer gives `end` the EOF type, so the parse stops at it.
    with keywords_parser.from_("class a end lambda b") as tree:
        end = tree[-1]
        assert end.symbol.type == keywords_parser.EOF
        assert end.symbol.text == "<EOF>"
        assert end.get_text() == "<EOF>"
        assert end.byte_span == (8, 11)
        assert end.source_text == "end"
        assert tree.get_text() == "classa<EOF>"
