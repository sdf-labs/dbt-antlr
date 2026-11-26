"""Lifetimes: trees and tokens that outlive their `with` block or thread."""

from __future__ import annotations

import gc
import threading
import traceback

import pytest


def test_leaked_references_raise_reference_error(csv_parser) -> None:
    with csv_parser.csvFile("a,b\nc\n") as tree:
        row = tree[0][0]
        comma = row[1]
        token = comma.symbol
        children = iter(row)
        next(children)
    for access in (
        lambda: tree.children,
        lambda: len(row),
        lambda: row[0],
        lambda: comma.parent,
        lambda: comma.kind,
        lambda: tree.get_text(),
        lambda: tree.source_text,
        lambda: tree.to_string_tree(),
        lambda: tree.syntax_errors,
        lambda: row.tokens_by_type(csv_parser.T__0),
        lambda: row.children_by_rule(csv_parser.RULE_field),
        lambda: token.text,
        lambda: token.line,
        lambda: token.type,
        lambda: next(children),
    ):
        with pytest.raises(ReferenceError, match="closed"):
            access()
    assert repr(tree) == "<Tree (closed)>"
    assert repr(token) == "<Token (closed)>"
    # Identity does not read the arena, so it keeps working.
    assert comma == comma
    assert hash(comma) == hash(comma)
    assert {tree, row, comma}


def test_tracebacks_format_closed_trees(csv_parser) -> None:
    def fail() -> None:
        with csv_parser.csvFile("a\nb\n") as tree:
            leaf = tree[0][0][0][0]
            raise RuntimeError(f"failed at {leaf!r}")

    with pytest.raises(RuntimeError) as error:
        fail()
    assert str(error.value) == "failed at <Tree terminal 'a'>"
    formatted = "".join(
        traceback.TracebackException.from_exception(error.value, capture_locals=True).format()
    )
    assert "<Tree (closed)>" in formatted


def test_trees_belong_to_their_thread(csv_parser) -> None:
    results = {}
    with csv_parser.csvFile("a\nb\n") as tree:

        def use_tree() -> None:
            results["repr"] = repr(tree)
            try:
                tree.children
            except ReferenceError as error:
                results["error"] = str(error)

        thread = threading.Thread(target=use_tree)
        thread.start()
        thread.join()
        assert tree.rule_name == "csvFile"
    assert results == {
        "repr": "<Tree (other thread)>",
        "error": "the parse tree belongs to a different thread",
    }


def test_contexts_exit_on_their_thread(csv_parser) -> None:
    context = csv_parser.csvFile("a\nb\n")
    tree = context.__enter__()
    errors = []

    def exit_context() -> None:
        try:
            context.__exit__(None, None, None)
        except RuntimeError as error:
            errors.append(str(error))

    thread = threading.Thread(target=exit_context)
    thread.start()
    thread.join()
    assert errors == ["a parse tree must be closed by the thread that parsed it"]
    assert tree.rule_name == "csvFile"
    context.__exit__(None, None, None)
    with pytest.raises(ReferenceError):
        tree.rule_name


def test_contexts_do_not_nest(csv_parser) -> None:
    context = csv_parser.csvFile("a\nb\n")
    with context as first:
        with pytest.raises(RuntimeError, match="already active"):
            context.__enter__()
        assert first.rule_name == "csvFile"
    with context as second:
        assert second.get_text() == "a\nb\n"
    assert first != second


def test_unclosed_sessions_are_freed_by_the_garbage_collector(csv_parser) -> None:
    for _ in range(100):
        tree = csv_parser.csvFile("a,b\n" * 10).__enter__()
        assert len(tree) == 10
    del tree
    gc.collect()


def test_many_parses(labels_parser) -> None:
    for index in range(1000):
        with labels_parser.e(f"{index} + x") as tree:
            assert tree[0].get_text() == str(index)
