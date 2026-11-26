"""Parse trees of dbt-antlr grammars, read in place from Rust arena memory.

A Rust parse session owns the memory of each parse tree. ``ParseContext``
creates the session when its ``with`` block starts and frees it when the
block ends. ``Tree`` and ``Token`` objects are views of the session memory:
they read node and token fields directly, without copies. After the ``with``
block, all views of the session raise ``ReferenceError``.
"""

from __future__ import annotations

import enum
import operator
from _thread import get_ident as _get_ident
from dataclasses import dataclass
from typing import Any, Iterator, Optional, Tuple, Union

# Values of antlr.h. `GrammarBundle` makes sure that its native module agrees.
_NODE_TAG_ERROR = 0
_NODE_TAG_TERMINAL = 1
_NODE_TAG_FIRST_RULE = 2
_TOKEN_EOF = -1
_TOKEN_TYPE_BITS = 22
_PARSE_STATUS_OK = 0
_PARSE_STATUS_INVALID_UTF8 = 4
_ERROR_ORIGIN_LEXER = 0

_HEADER_CONSTANTS = {
    "ANTLR_NODE_TAG_ERROR": _NODE_TAG_ERROR,
    "ANTLR_NODE_TAG_TERMINAL": _NODE_TAG_TERMINAL,
    "ANTLR_NODE_TAG_FIRST_RULE": _NODE_TAG_FIRST_RULE,
    "ANTLR_TOKEN_EOF": _TOKEN_EOF,
    "ANTLR_TOKEN_TYPE_BITS": _TOKEN_TYPE_BITS,
    "ANTLR_PARSE_STATUS_OK": _PARSE_STATUS_OK,
    "ANTLR_PARSE_STATUS_INVALID_UTF8": _PARSE_STATUS_INVALID_UTF8,
    "ANTLR_ERROR_ORIGIN_LEXER": _ERROR_ORIGIN_LEXER,
}

# The `type_and_channel` bits of the token type of EOF.
_EOF_TYPE_BITS = _TOKEN_EOF & ((1 << _TOKEN_TYPE_BITS) - 1)

_new = object.__new__

Input = Union[str, bytes, bytearray, memoryview]


class NodeKind(enum.Enum):
    """The kind of a parse tree node."""

    ERROR = "error"
    """A leaf with a token that error recovery skipped or conjured."""
    TERMINAL = "terminal"
    """A leaf with a token that a rule matched."""
    RULE = "rule"
    """A node of a parser rule."""


@dataclass(frozen=True)
class SyntaxDiagnostic:
    """A syntax error of a parse.

    The diagnostic is a plain Python value: it stays valid after the parse
    tree is closed.
    """

    line: int
    """The line of the error (1-based)."""
    column: int
    """The column of the error in code points (0-based), or -1."""
    message: str
    """The error message, for example ``missing ')' at '}'``."""
    origin: str
    """``"lexer"`` when the lexer found no token, else ``"parser"``."""
    token_type: Optional[int]
    """The type of the offending token, or ``None`` for lexer errors."""
    token_index: Optional[int]
    """The token stream index of the offending token, or ``None``."""
    start_byte: Optional[int]
    """The UTF-8 byte offset where the offending input starts, or ``None``."""
    stop_byte: Optional[int]
    """The byte offset of the last byte of the offending token, or ``None``."""

    def __str__(self) -> str:
        return f"line {self.line}:{self.column} {self.message}"


class ParseError(Exception):
    """A parse that produced no usable tree.

    ``errors`` holds the syntax errors that the parse reported before it
    stopped.
    """

    def __init__(self, message: str, errors: Tuple[SyntaxDiagnostic, ...] = ()) -> None:
        super().__init__(message)
        self.errors = errors


class ParseSyntaxError(ParseError):
    """A parse with syntax errors, when the parse did not use recovery.

    ``errors`` holds all syntax errors of the parse.
    """

    def __init__(self, errors: Tuple[SyntaxDiagnostic, ...]) -> None:
        first = errors[0]
        more = f" (and {len(errors) - 1} more)" if len(errors) > 1 else ""
        super().__init__(f"{first}{more}", errors)


def _str(ffi: Any, view: Any) -> Optional[str]:
    """The Python string of an ``AntlrStr`` view, or ``None``."""
    if not view.ptr:
        return None
    return ffi.unpack(view.ptr, view.len).decode("utf-8")


def _strs(ffi: Any, views: Any, count: int) -> Tuple[Optional[str], ...]:
    return tuple(_str(ffi, views[index]) for index in range(count))


class GrammarBundle:
    """The grammars of one native extension module.

    A native module is a CFFI extension module (for example ``_native``) that
    links a Rust bundle crate. The crate names its grammars with
    ``dbt_antlr_ffi::export_grammars!``.
    """

    __slots__ = ("ffi", "lib", "_grammars")

    _loaded: "dict[int, Tuple[Any, GrammarBundle]]" = {}

    def __init__(self, ffi: Any, lib: Any) -> None:
        for name, value in _HEADER_CONSTANTS.items():
            if getattr(lib, name) != value:
                raise ImportError(
                    f"the native module does not match dbt_antlr: {name} is "
                    f"{getattr(lib, name)}, not {value}"
                )
        self.ffi = ffi
        self.lib = lib
        self._grammars: "dict[str, Grammar]" = {}

    @classmethod
    def load(cls, native: Any) -> GrammarBundle:
        """Returns the bundle of the native module ``native``.

        All calls with the same module return the same bundle.
        """
        key = id(native)
        loaded = cls._loaded.get(key)
        if loaded is None or loaded[0] is not native:
            loaded = (native, cls(native.ffi, native.lib))
            cls._loaded[key] = loaded
        return loaded[1]

    def grammar(self, name: str, *, rule_names: Optional[Tuple[str, ...]] = None) -> Grammar:
        """Returns the grammar that the bundle exports as ``name``.

        When ``rule_names`` is given, the grammar must have these rules: a
        generated module passes its rule names, so that a stale native module
        fails at import time.
        """
        grammar = self._grammars.get(name)
        if grammar is None:
            encoded = name.encode("utf-8")
            grammar_id = self.lib.antlr_grammar_lookup(encoded, len(encoded))
            if grammar_id < 0:
                raise ImportError(
                    f"the native module exports no grammar {name!r}: export it "
                    f"with dbt_antlr_ffi::export_grammars!"
                )
            grammar = Grammar(self, name, grammar_id)
            self._grammars[name] = grammar
        if rule_names is not None and tuple(rule_names) != grammar.rule_names:
            raise ImportError(
                f"the rules of grammar {name!r} in the native module do not match "
                f"the generated Python module: regenerate both from the same grammar"
            )
        return grammar

    def set_arena_allocation_limit(self, limit_bytes: int) -> None:
        """Limits the arena memory of each parse that starts after this call.

        The limit applies to all grammars of the bundle. A parse that goes
        above the limit raises ``ParseError``. ``0`` removes the limit.

        Raises ``RuntimeError`` when the bundle was built without the
        ``arena-allocation-limit`` feature of ``dbt-antlr-ffi``.
        """
        if limit_bytes < 0:
            raise ValueError("the limit cannot be negative")
        if not self.lib.antlr_set_arena_allocation_limit(limit_bytes):
            raise RuntimeError(
                "the bundle was built without the arena-allocation-limit feature "
                "of dbt-antlr-ffi"
            )


class Grammar:
    """A grammar of a ``GrammarBundle``."""

    __slots__ = (
        "bundle",
        "name",
        "grammar_id",
        "rule_names",
        "alt_labels",
        "entry_rules",
        "literal_names",
        "symbolic_names",
        "_rule_indexes",
    )

    def __init__(self, bundle: GrammarBundle, name: str, grammar_id: int) -> None:
        ffi = bundle.ffi
        info = bundle.lib.antlr_grammar_info(grammar_id)
        rules = [info.rules[index] for index in range(info.rule_count)]
        self.bundle = bundle
        self.name = name
        self.grammar_id = grammar_id
        self.rule_names: Tuple[str, ...] = tuple(_str(ffi, rule.name) or "" for rule in rules)
        self.alt_labels: Tuple[Tuple[str, ...], ...] = tuple(
            tuple(
                _str(ffi, rule.alt_labels[index]) or "" for index in range(rule.alt_label_count)
            )
            for rule in rules
        )
        self.entry_rules: Tuple[bool, ...] = tuple(bool(rule.is_entry) for rule in rules)
        self.literal_names = _strs(ffi, info.literal_names, info.literal_name_count)
        self.symbolic_names = _strs(ffi, info.symbolic_names, info.symbolic_name_count)
        self._rule_indexes = {rule: index for index, rule in enumerate(self.rule_names)}

    def __repr__(self) -> str:
        return f"<Grammar {self.name!r}>"

    def rule_index(self, rule_name: str) -> int:
        """The index of the rule ``rule_name``."""
        return self._rule_indexes[rule_name]

    def token_name(self, token_type: int) -> Optional[str]:
        """The symbolic name (``ID``) or the literal name (``'+'``) of a
        token type, or ``None``."""
        if token_type == _TOKEN_EOF:
            return "EOF"
        for names in (self.symbolic_names, self.literal_names):
            if 0 <= token_type < len(names) and names[token_type] is not None:
                return names[token_type]
        return None

    def parse(self, rule_index: int, input: Input, *, recovery: bool = False) -> ParseContext:
        """Returns a context that parses ``input`` from the rule
        ``rule_index``. See ``ParseContext``."""
        return ParseContext(self, rule_index, input, recovery=recovery)


def _encode(input: Input) -> Tuple[bytes, Optional[str]]:
    """The UTF-8 bytes of ``input``, and ``input`` as a string when it is
    ASCII (then string offsets are byte offsets)."""
    if isinstance(input, str):
        return input.encode("utf-8"), input if input.isascii() else None
    data = input if isinstance(input, bytes) else bytes(input)
    return data, data.decode("ascii") if data.isascii() else None


class ParseContext:
    """A parse, as a context manager.

    ``with context as tree`` parses the input and gives the root ``Tree``.
    The tree is valid until the ``with`` block ends.

    Without ``recovery``, a parse with syntax errors raises
    ``ParseSyntaxError``. With ``recovery``, the tree contains the
    recovered parse (with error nodes) and ``tree.syntax_errors`` tells the
    errors. A parse that produces no tree raises ``ParseError``. Bytes that
    are not valid UTF-8 raise ``UnicodeDecodeError``.
    """

    __slots__ = ("_grammar", "_rule_index", "_input", "_recovery", "_session")

    def __init__(
        self, grammar: Grammar, rule_index: int, input: Input, *, recovery: bool = False
    ) -> None:
        if not isinstance(input, (str, bytes, bytearray, memoryview)):
            raise TypeError(f"the input must be str or bytes, not {type(input).__name__}")
        rule_index = operator.index(rule_index)
        if not 0 <= rule_index < len(grammar.rule_names):
            raise ValueError(f"grammar {grammar.name!r} has no rule with index {rule_index}")
        if not grammar.entry_rules[rule_index]:
            raise ValueError(
                f"rule {grammar.rule_names[rule_index]!r} has arguments: a parse cannot "
                f"start at it"
            )
        self._grammar = grammar
        self._rule_index = rule_index
        self._input = input
        self._recovery = recovery
        self._session: Optional[_Session] = None

    def __enter__(self) -> Tree:
        if self._session is not None:
            raise RuntimeError("the parse context is already active")
        data, text = _encode(self._input)
        session = _Session(self._grammar, self._rule_index, data, text)
        if session.status != _PARSE_STATUS_OK:
            status, message, errors = session.status, session.message, session.syntax_errors
            session.close()
            if status == _PARSE_STATUS_INVALID_UTF8:
                data.decode("utf-8")
            raise ParseError(message or "the parse failed", errors)
        if session.syntax_errors and not self._recovery:
            errors = session.syntax_errors
            session.close()
            raise ParseSyntaxError(errors)
        self._session = session
        return _tree(session, session._root, None)

    def __exit__(self, *exc_info: Any) -> None:
        session = self._session
        if session is not None:
            session.close()
            self._session = None


def _diagnostic(ffi: Any, error: Any) -> SyntaxDiagnostic:
    lexer = error.origin == _ERROR_ORIGIN_LEXER
    return SyntaxDiagnostic(
        line=error.line,
        column=error.column,
        message=_str(ffi, error.message) or "",
        origin="lexer" if lexer else "parser",
        token_type=None if lexer else error.token_type,
        token_index=error.token_index if error.token_index >= 0 else None,
        start_byte=error.start if error.start >= 0 else None,
        stop_byte=error.stop if error.stop >= 0 else None,
    )


class _Session:
    """A live Rust parse session and the input that it borrows."""

    __slots__ = (
        "grammar",
        "status",
        "message",
        "syntax_errors",
        "_ffi",
        "_lib",
        "_ptr",
        "_thread_id",
        "_data",
        "_text",
        "_len",
        "_buffer",
        "_primary",
        "_root",
        "_len_out",
        "_line_out",
        "_column_out",
    )

    def __init__(self, grammar: Grammar, rule_index: int, data: bytes, text: Optional[str]) -> None:
        ffi = grammar.bundle.ffi
        lib = grammar.bundle.lib
        self.grammar = grammar
        self._ffi = ffi
        self._lib = lib
        self._thread_id = _get_ident()
        self._data: Optional[bytes] = data
        self._text = text
        self._len = len(data)
        # `from_buffer` pins `data`: the session borrows the bytes until
        # `close`.
        self._buffer = ffi.from_buffer("char[]", data)
        raw = lib.antlr_parse(grammar.grammar_id, rule_index, self._buffer, len(data))
        if not raw:
            raise ParseError("the native parser failed unexpectedly")
        self._ptr: Any = ffi.gc(raw, lib.antlr_session_free)
        info = lib.antlr_session_info(raw)
        self.status: int = info.status
        self.message = _str(ffi, info.message)
        self._primary = info.primary_source
        self._root = info.root
        errors = info.syntax_errors
        self.syntax_errors = tuple(
            _diagnostic(ffi, errors[index]) for index in range(info.syntax_error_count)
        )
        self._len_out = ffi.new("size_t *")
        self._line_out = ffi.new("uint32_t *")
        self._column_out = ffi.new("int32_t *")

    def close(self) -> None:
        """Frees the session and unpins the input.

        Only the thread that parsed can close: the thread can read the
        session at any time. The ``ffi.gc`` finalizer can free the session on
        any thread, because it runs only when no view of the session is left.
        """
        ptr = self._ptr
        if ptr is None:
            return
        if _get_ident() != self._thread_id:
            raise RuntimeError("a parse tree must be closed by the thread that parsed it")
        self._ptr = None
        self._ffi.release(ptr)
        self._buffer = None
        self._data = None
        self._text = None

    def _check(self) -> None:
        if self._ptr is None:
            raise ReferenceError(
                "the parse tree is closed: use it only in the `with` block of its ParseContext"
            )
        if _get_ident() != self._thread_id:
            raise ReferenceError("the parse tree belongs to a different thread")

    def _state(self) -> Optional[str]:
        """Why the session cannot be read, or ``None`` when it can."""
        if self._ptr is None:
            return "closed"
        if _get_ident() != self._thread_id:
            return "other thread"
        return None

    def _end(self, stop: int) -> int:
        """The byte offset after the character whose last byte is ``stop``."""
        end = stop + 1
        if self._text is None:
            data = self._data
            length = self._len
            while end < length and (data[end] & 0xC0) == 0x80:  # type: ignore[index]
                end += 1
        return end

    def _slice(self, begin: int, end: int) -> str:
        text = self._text
        if text is not None:
            return text[begin:end]
        return self._data[begin:end].decode("utf-8")  # type: ignore[index]

    def _span(self, first: Any, last: Any) -> Optional[Tuple[int, int]]:
        begin = first.start
        if begin < 0:
            return None
        return begin, max(self._end(last.stop), begin)

    def _token_text(self, token: Any) -> str:
        # The fast path slices the input for the tokens that `CommonToken`
        # slices it for: tokens of the primary source that are not EOF.
        if token.source == self._primary and (
            token.type_and_channel & _EOF_TYPE_BITS
        ) != _EOF_TYPE_BITS:
            begin = token.start
            stop = token.stop
            if 0 <= begin <= stop < self._len:
                text = self._text
                if text is not None:
                    return text[begin : stop + 1]
                return self._data[begin : self._end(stop)].decode("utf-8")  # type: ignore[index]
        text_ptr = self._lib.antlr_token_text_fallback(token, self._len_out)
        if not text_ptr:
            raise RuntimeError("the native parser cannot compute the token text")
        return self._ffi.unpack(text_ptr, self._len_out[0]).decode("utf-8")


def _tree(session: _Session, ptr: Any, parent: Any) -> Tree:
    tree = _new(Tree)
    tree._session = session
    tree._ptr = ptr
    tree._parent = parent
    return tree


def _token(session: _Session, ptr: Any) -> Token:
    token = _new(Token)
    token._session = session
    token._ptr = ptr
    return token


def _children(session: _Session, node: Any) -> Iterator[Tree]:
    lib = session._lib
    slots = lib.antlr_rule_children(node)
    for index in range(lib.antlr_rule_child_count(node)):
        session._check()
        yield _tree(session, slots[index], node)


def _tokens_by_type(
    session: _Session, children: Iterator[Tree], token_type: int
) -> Iterator[Token]:
    lib = session._lib
    for child in children:
        node = child._ptr
        if node.node_tag == _NODE_TAG_TERMINAL:
            symbol = lib.antlr_leaf_symbol(node)
            if lib.antlr_token_type(symbol) == token_type:
                yield _token(session, symbol)


class Tree:
    """A node of a parse tree: a rule node, a terminal leaf, or an error leaf.

    A rule node is a sequence of its children: ``len(tree)``, ``tree[i]``,
    and ``iter(tree)`` work on the children. A leaf has no children.

    Two ``Tree`` objects are equal when they view the same node.
    """

    __slots__ = ("_session", "_ptr", "_parent")

    _session: _Session
    _ptr: Any
    _parent: Any

    def __init__(self) -> None:
        raise TypeError("parse trees come from a ParseContext")

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Tree):
            return NotImplemented
        return other._session is self._session and other._ptr == self._ptr

    def __hash__(self) -> int:
        return hash(self._ptr)

    def __bool__(self) -> bool:
        return True

    def __repr__(self) -> str:
        session = self._session
        state = session._state()
        if state is not None:
            return f"<Tree ({state})>"
        node = self._ptr
        tag = node.node_tag
        if tag >= _NODE_TAG_FIRST_RULE:
            return f"<Tree rule {session.grammar.rule_names[tag - _NODE_TAG_FIRST_RULE]}>"
        kind = "terminal" if tag == _NODE_TAG_TERMINAL else "error"
        text = session._token_text(session._lib.antlr_leaf_symbol(node))
        return f"<Tree {kind} {text!r}>"

    @property
    def kind(self) -> NodeKind:
        """The kind of the node."""
        self._session._check()
        tag = self._ptr.node_tag
        if tag >= _NODE_TAG_FIRST_RULE:
            return NodeKind.RULE
        return NodeKind.TERMINAL if tag == _NODE_TAG_TERMINAL else NodeKind.ERROR

    @property
    def is_rule(self) -> bool:
        """True for a rule node."""
        self._session._check()
        return self._ptr.node_tag >= _NODE_TAG_FIRST_RULE

    @property
    def is_terminal(self) -> bool:
        """True for a terminal leaf."""
        self._session._check()
        return self._ptr.node_tag == _NODE_TAG_TERMINAL

    @property
    def is_error(self) -> bool:
        """True for an error leaf."""
        self._session._check()
        return self._ptr.node_tag == _NODE_TAG_ERROR

    @property
    def rule_index(self) -> Optional[int]:
        """The rule index of a rule node, or ``None`` for a leaf."""
        self._session._check()
        tag = self._ptr.node_tag
        return tag - _NODE_TAG_FIRST_RULE if tag >= _NODE_TAG_FIRST_RULE else None

    @property
    def rule_name(self) -> Optional[str]:
        """The rule name of a rule node, or ``None`` for a leaf."""
        session = self._session
        session._check()
        tag = self._ptr.node_tag
        if tag < _NODE_TAG_FIRST_RULE:
            return None
        return session.grammar.rule_names[tag - _NODE_TAG_FIRST_RULE]

    @property
    def alt_label(self) -> Optional[str]:
        """The label (``# label``) of the alternative that a rule node
        matched, or ``None``."""
        session = self._session
        session._check()
        node = self._ptr
        tag = node.node_tag
        if tag < _NODE_TAG_FIRST_RULE:
            return None
        labels = session.grammar.alt_labels[tag - _NODE_TAG_FIRST_RULE]
        label_tag = node.label_tag
        return labels[label_tag] if label_tag < len(labels) else None

    @property
    def invoking_state(self) -> int:
        """The ATN state that invoked the rule of a rule node, or -1."""
        self._session._check()
        return self._ptr.invoking_state

    @property
    def parent(self) -> Optional[Tree]:
        """The parent rule node, or ``None`` for the root."""
        session = self._session
        session._check()
        node = self._ptr
        if node.node_tag >= _NODE_TAG_FIRST_RULE:
            parent = session._lib.antlr_rule_parent(node)
        else:
            parent = self._parent
        return _tree(session, parent, None) if parent else None

    def __len__(self) -> int:
        session = self._session
        session._check()
        node = self._ptr
        if node.node_tag < _NODE_TAG_FIRST_RULE:
            return 0
        return session._lib.antlr_rule_child_count(node)

    def __getitem__(self, index: Any) -> Any:
        if isinstance(index, slice):
            return self.children[index]
        session = self._session
        session._check()
        index = operator.index(index)
        node = self._ptr
        lib = session._lib
        count = lib.antlr_rule_child_count(node) if node.node_tag >= _NODE_TAG_FIRST_RULE else 0
        if index < 0:
            index += count
        if not 0 <= index < count:
            raise IndexError("child index out of range")
        return _tree(session, lib.antlr_rule_child(node, index), node)

    def __iter__(self) -> Iterator[Tree]:
        session = self._session
        session._check()
        node = self._ptr
        if node.node_tag < _NODE_TAG_FIRST_RULE:
            return iter(())
        return _children(session, node)

    @property
    def children(self) -> Tuple[Tree, ...]:
        """The children of a rule node; empty for a leaf."""
        session = self._session
        session._check()
        node = self._ptr
        if node.node_tag < _NODE_TAG_FIRST_RULE:
            return ()
        lib = session._lib
        slots = lib.antlr_rule_children(node)
        count = lib.antlr_rule_child_count(node)
        return tuple([_tree(session, slots[index], node) for index in range(count)])

    def child_by_rule(self, rule_index: int, i: int = 0) -> Optional[Tree]:
        """The ``i``-th child rule node of the rule ``rule_index``, or
        ``None``."""
        for child in self.children_by_rule(rule_index):
            if i == 0:
                return child
            i -= 1
        return None

    def children_by_rule(self, rule_index: int) -> Iterator[Tree]:
        """The child rule nodes of the rule ``rule_index``."""
        tag = rule_index + _NODE_TAG_FIRST_RULE
        return (child for child in self if child._ptr.node_tag == tag)

    def token_by_type(self, token_type: int, i: int = 0) -> Optional[Token]:
        """The token of the ``i``-th terminal child with the type
        ``token_type``, or ``None``."""
        for token in self.tokens_by_type(token_type):
            if i == 0:
                return token
            i -= 1
        return None

    def tokens_by_type(self, token_type: int) -> Iterator[Token]:
        """The tokens of the terminal children with the type ``token_type``."""
        return _tokens_by_type(self._session, iter(self), token_type)

    @property
    def symbol(self) -> Optional[Token]:
        """The token of a leaf, or ``None`` for a rule node."""
        session = self._session
        session._check()
        node = self._ptr
        if node.node_tag >= _NODE_TAG_FIRST_RULE:
            return None
        return _token(session, session._lib.antlr_leaf_symbol(node))

    @property
    def start_token(self) -> Optional[Token]:
        """The first token of the node, or ``None``."""
        session = self._session
        session._check()
        node = self._ptr
        lib = session._lib
        if node.node_tag < _NODE_TAG_FIRST_RULE:
            return _token(session, lib.antlr_leaf_symbol(node))
        start = lib.antlr_rule_start(node)
        return _token(session, start) if start else None

    @property
    def stop_token(self) -> Optional[Token]:
        """The last token of the node, or ``None``. When a rule matched no
        token, its stop token comes before its start token."""
        session = self._session
        session._check()
        node = self._ptr
        lib = session._lib
        if node.node_tag < _NODE_TAG_FIRST_RULE:
            return _token(session, lib.antlr_leaf_symbol(node))
        stop = lib.antlr_rule_stop(node)
        return _token(session, stop) if stop else None

    @property
    def byte_span(self) -> Optional[Tuple[int, int]]:
        """The UTF-8 byte range ``(begin, end)`` of the input that the node
        covers, with ``end`` exclusive, or ``None`` when the node has no
        input (for example a conjured ``<missing ...>`` token)."""
        session = self._session
        session._check()
        node = self._ptr
        lib = session._lib
        if node.node_tag < _NODE_TAG_FIRST_RULE:
            symbol = lib.antlr_leaf_symbol(node)
            return session._span(symbol, symbol)
        start = lib.antlr_rule_start(node)
        if not start:
            return None
        stop = lib.antlr_rule_stop(node)
        if not stop or stop.token_index < start.token_index:
            begin = start.start
            return (begin, begin) if begin >= 0 else None
        return session._span(start, stop)

    @property
    def source_text(self) -> str:
        """The input that the node covers, with hidden tokens such as
        whitespace."""
        span = self.byte_span
        if span is None:
            return ""
        return self._session._slice(*span)

    def get_text(self) -> str:
        """The texts of the leaves of the node, concatenated.

        Hidden tokens (for example whitespace) are not leaves, so the text
        does not contain them. Conjured leaves add their text (for example
        ``<missing ')'>``), and the EOF leaf adds ``<EOF>``.
        """
        session = self._session
        session._check()
        lib = session._lib
        leaf_symbol = lib.antlr_leaf_symbol
        token_text = session._token_text
        node = self._ptr
        if node.node_tag < _NODE_TAG_FIRST_RULE:
            return token_text(leaf_symbol(node))
        parts = []
        stack = [node]
        while stack:
            node = stack.pop()
            if node.node_tag < _NODE_TAG_FIRST_RULE:
                parts.append(token_text(leaf_symbol(node)))
                continue
            slots = lib.antlr_rule_children(node)
            last = lib.antlr_rule_child_count(node) - 1
            stack.extend([slots[index] for index in range(last, -1, -1)])
        return "".join(parts)

    @property
    def syntax_errors(self) -> Tuple[SyntaxDiagnostic, ...]:
        """The syntax errors of the parse that produced the tree."""
        session = self._session
        session._check()
        return session.syntax_errors

    def to_string_tree(self) -> str:
        """The subtree in LISP form, for example ``(s (e 1) <EOF>)``."""
        session = self._session
        session._check()
        lib = session._lib
        length = session._len_out
        text_ptr = lib.antlr_node_to_string_tree(session.grammar.grammar_id, self._ptr, length)
        if not text_ptr:
            raise RuntimeError("the native parser cannot format the tree")
        size = length[0]
        try:
            return session._ffi.unpack(text_ptr, size).decode("utf-8")
        finally:
            lib.antlr_string_free(text_ptr, size)


class Token:
    """A token of a parse tree.

    Two ``Token`` objects are equal when they view the same token.
    """

    __slots__ = ("_session", "_ptr")

    _session: _Session
    _ptr: Any

    def __init__(self) -> None:
        raise TypeError("tokens come from a parse tree")

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Token):
            return NotImplemented
        return other._session is self._session and other._ptr == self._ptr

    def __hash__(self) -> int:
        return hash(self._ptr)

    def __repr__(self) -> str:
        session = self._session
        state = session._state()
        if state is not None:
            return f"<Token ({state})>"
        token = self._ptr
        token_type = session._lib.antlr_token_type(token)
        name = session.grammar.token_name(token_type) or str(token_type)
        return f"<Token {name} {session._token_text(token)!r}>"

    @property
    def type(self) -> int:
        """The token type (``-1`` for EOF)."""
        session = self._session
        session._check()
        return session._lib.antlr_token_type(self._ptr)

    @property
    def channel(self) -> int:
        """The channel of the token (0 is the default channel)."""
        session = self._session
        session._check()
        return session._lib.antlr_token_channel(self._ptr)

    @property
    def token_index(self) -> int:
        """The index of the token in the token stream, or -1 for a conjured
        token."""
        self._session._check()
        return self._ptr.token_index

    @property
    def start_byte(self) -> int:
        """The UTF-8 byte offset of the first byte of the token, or -1 for a
        conjured token."""
        self._session._check()
        return self._ptr.start

    @property
    def stop_byte(self) -> int:
        """The UTF-8 byte offset of the last byte of the token (inclusive), or
        -1 for a conjured token."""
        self._session._check()
        return self._ptr.stop

    @property
    def byte_span(self) -> Optional[Tuple[int, int]]:
        """The UTF-8 byte range ``(begin, end)`` of the token, with ``end``
        exclusive, or ``None`` for a conjured token."""
        session = self._session
        session._check()
        return session._span(self._ptr, self._ptr)

    @property
    def text(self) -> str:
        """The text of the token: ``<EOF>`` for EOF, the conjured text for a
        conjured token (for example ``<missing ')'>``), and the input slice
        for other tokens."""
        session = self._session
        session._check()
        return session._token_text(self._ptr)

    @property
    def line(self) -> int:
        """The line of the first character of the token (1-based)."""
        return self._location()[0]

    @property
    def column(self) -> int:
        """The column of the first character of the token in code points
        (0-based)."""
        return self._location()[1]

    def _location(self) -> Tuple[int, int]:
        session = self._session
        session._check()
        if not session._lib.antlr_token_location(self._ptr, session._line_out, session._column_out):
            raise RuntimeError("the native parser cannot compute the token location")
        return session._line_out[0], session._column_out[0]
