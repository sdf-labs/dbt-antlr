"""Navigation of parse trees."""

from __future__ import annotations

import pytest

from dbt_antlr import NodeKind, Tree


def test_rule_nodes_and_leaves(csv_parser) -> None:
    with csv_parser.csvFile("a,b\nc\n") as tree:
        assert isinstance(tree, Tree)
        assert tree.kind is NodeKind.RULE
        assert tree.is_rule and not tree.is_terminal and not tree.is_error
        assert tree.rule_index == csv_parser.RULE_csvFile
        assert tree.rule_name == "csvFile"
        assert tree.parent is None
        assert tree.symbol is None
        assert tree.alt_label is None

        header_row = tree[0][0]
        assert header_row.rule_name == "row"
        comma = header_row[1]
        assert comma.kind is NodeKind.TERMINAL
        assert comma.is_terminal
        assert comma.rule_index is None and comma.rule_name is None
        assert len(comma) == 0 and list(comma) == [] and comma.children == ()
        assert bool(comma)
        assert comma.symbol.text == ","
        assert comma.symbol.type == csv_parser.T__0


def test_children_as_a_sequence(csv_parser) -> None:
    with csv_parser.csvFile("a,b,c\nd\n") as tree:
        row = tree[0][0]
        assert len(row) == 6
        texts = [child.get_text() for child in row]
        assert texts == ["a", ",", "b", ",", "c", "\n"]
        assert [child.get_text() for child in row.children] == texts
        assert row[-1] == row[5]
        assert row[-6] == row[0]
        assert [child.get_text() for child in row[1:4]] == [",", "b", ","]
        with pytest.raises(IndexError):
            row[6]
        with pytest.raises(IndexError):
            row[-7]
        with pytest.raises(TypeError):
            row["0"]


def test_inline_and_spilled_children(csv_parser) -> None:
    # A rule node holds up to three children inline, and more children in
    # an arena array.
    with csv_parser.csvFile("a\nb,c,d,e,f\n") as tree:
        inline_row = tree[0][0]
        spilled_row = tree[1]
        assert len(inline_row) == 2
        assert len(spilled_row) == 10
        assert [child.get_text() for child in spilled_row] == [
            "b", ",", "c", ",", "d", ",", "e", ",", "f", "\n",
        ]
        for row in (inline_row, spilled_row):
            for child in row:
                assert child.parent == row


def test_parent_identity_and_hashing(csv_parser) -> None:
    with csv_parser.csvFile("a,b\nc\n") as tree:
        row = tree[0][0]
        field = row[0]
        assert field.parent == row
        assert field.parent.parent.parent == tree
        assert row[1].parent == row
        assert row[0] == row[0] and row[0] is not row[0]
        assert row[0] != row[2]
        assert hash(row[0]) == hash(row[0])
        seen = {child: index for index, child in enumerate(row)}
        assert seen[row[2]] == 2
        assert len({child.parent for child in row}) == 1
        assert tree != "csvFile"


def test_children_and_tokens_by_kind(csv_parser) -> None:
    with csv_parser.csvFile('a,"b",\nx\n') as tree:
        row = tree[0][0]
        fields = list(row.children_by_rule(csv_parser.RULE_field))
        assert [field.get_text() for field in fields] == ["a", '"b"', ""]
        assert row.child_by_rule(csv_parser.RULE_field, 1) == fields[1]
        assert row.child_by_rule(csv_parser.RULE_field, 3) is None
        assert row.child_by_rule(csv_parser.RULE_row) is None

        commas = list(row.tokens_by_type(csv_parser.T__0))
        assert [comma.start_byte for comma in commas] == [1, 5]
        assert row.token_by_type(csv_parser.T__0, 1) == commas[1]
        assert row.token_by_type(csv_parser.T__0, 2) is None
        assert fields[1].token_by_type(csv_parser.STRING).text == '"b"'
        assert fields[1].token_by_type(csv_parser.TEXT) is None


def test_alt_labels(labels_parser) -> None:
    with labels_parser.e("1 + a * (2)") as tree:
        assert tree.rule_name == "e"
        assert tree.alt_label == "add"
        assert tree[0].alt_label == "anInt"
        product = tree[2]
        assert product.alt_label == "mult"
        assert product[0].alt_label == "anID"
        assert product[2].alt_label == "parens"
        assert product[1].alt_label is None
    with labels_parser.s("{ 1 }") as tree:
        assert tree.alt_label is None
        assert tree[1].rule_name == "blk"


def test_get_text_and_source_text(csv_parser) -> None:
    # Whitespace goes to the hidden channel: it is not a leaf, so get_text()
    # does not contain it, but source_text slices the input.
    text = "a , b\nc ,  d\n"
    with csv_parser.csvFile(text) as tree:
        assert tree.get_text() == "a,b\nc,d\n"
        assert tree.source_text == text
        row = tree[1]
        assert row.get_text() == "c,d\n"
        assert row.source_text == "c ,  d\n"
        assert row.byte_span == (6, 13)
        assert row.start_token.text == "c"
        assert row.stop_token.text == "\n"


def test_empty_rule(csv_parser) -> None:
    with csv_parser.csvFile("a,\nb\n") as tree:
        empty_field = tree[0][0][2]
        assert empty_field.rule_name == "field"
        assert len(empty_field) == 0
        assert empty_field.get_text() == ""
        assert empty_field.source_text == ""
        assert empty_field.byte_span == (2, 2)
        assert empty_field.stop_token.token_index < empty_field.start_token.token_index


def test_to_string_tree(csv_parser, labels_parser) -> None:
    with csv_parser.csvFile("a,b\nc\n") as tree:
        assert tree.to_string_tree() == (
            "(csvFile (hdr (row (field a) , (field b) \\n)) (row (field c) \\n))"
        )
        assert tree[0][0][0].to_string_tree() == "(field a)"
        assert tree[0][0][1].to_string_tree() == ","
    with labels_parser.s("{ 1 + 2 }") as tree:
        assert tree.to_string_tree() == "(s { (blk (s (e (e 1) + (e 2)))) })"


def test_eof_leaf(keywords_parser) -> None:
    with keywords_parser.from_("class a") as tree:
        eof = tree[-1]
        assert eof.is_terminal
        assert eof.symbol.type == keywords_parser.EOF
        assert eof.get_text() == "<EOF>"
        assert eof.source_text == ""
        assert eof.byte_span == (7, 7)
        assert tree.get_text() == "classa<EOF>"
        assert tree.source_text == "class a"


def test_trees_cannot_be_created_directly() -> None:
    with pytest.raises(TypeError):
        Tree()
