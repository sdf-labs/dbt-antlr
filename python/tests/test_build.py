"""The CFFI declarations that dbt_antlr.build extracts from antlr.h."""

from __future__ import annotations

import cffi
import pytest

from dbt_antlr import build


def test_cdef_declares_the_abi() -> None:
    text = build.cdef()
    ffi = cffi.FFI()
    ffi.cdef(text)
    assert ffi.sizeof("AntlrToken") == 24
    assert ffi.offsetof("AntlrTreeNode", "body") == 8
    assert ffi.sizeof("AntlrRuleBody") == 56
    assert "antlr_parse(" in text
    assert (
        "const AntlrTreeNode *antlr_rule_child(const AntlrTreeNode *node, uint32_t index);"
        in text
    )


def test_cdef_evaluates_conditionals_and_inline_bodies() -> None:
    header = """
#ifndef GUARD
#define GUARD
#include <stdint.h>
#define VALUE -2
#ifdef __cplusplus
extern "C" {
#else
int c_only(void);
#endif
#ifdef __cplusplus
}
#endif
static inline int nested(int x) {
  if (x) { return 1; }
  return 0;
}
int after(void);
#endif
"""
    text = build.cdef(header)
    assert "#define VALUE -2" in text
    assert "int c_only(void);" in text
    assert "int nested(int x);" in text
    assert "int after(void);" in text
    assert "extern" not in text and "GUARD" not in text and "return" not in text
    cffi.FFI().cdef(text)


def test_cdef_rejects_unknown_directives() -> None:
    with pytest.raises(ValueError, match="unsupported directive"):
        build.cdef("#if 1\n#endif\n")
