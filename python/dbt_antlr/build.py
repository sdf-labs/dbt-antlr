"""Builds the native extension module of a grammar bundle with CFFI.

A grammar bundle is a Rust crate with ``crate-type = ["staticlib"]`` that
exports grammars with ``dbt_antlr_ffi::export_grammars!``. This module links
the static library into a CFFI extension module (API mode, ``abi3``):

    from dbt_antlr import build

    library = build.build_staticlib("my-bundle", manifest_path="Cargo.toml")
    build.compile_bundle("my_package._native", library, output_dir="build")

The generated ``<name>_parser.py`` modules import the extension module as
``._native``, so put them into the same package.

Building needs ``cffi`` and ``setuptools``. Using the extension module needs
only ``cffi``.
"""

from __future__ import annotations

import contextlib
import json
import os
import platform
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Iterable, Iterator, List, Optional, Sequence, Union

PathLike = Union[str, "os.PathLike[str]"]

_HEADER = "antlr.h"


def header_dir() -> Path:
    """The directory of ``antlr.h``."""
    return Path(__file__).resolve().parent


def header_path() -> Path:
    """The path of ``antlr.h``, the C declaration of the ABI."""
    return header_dir() / _HEADER


def cdef(header: Optional[str] = None) -> str:
    """The declarations of ``antlr.h`` in the form that ``FFI.cdef`` accepts.

    The function removes comments, include directives, include guards, and
    ``__cplusplus`` blocks, and replaces the bodies of the ``static inline``
    accessors with prototypes. CFFI then calls the accessors of the real
    header, which the C compiler inlines.
    """
    text = header_path().read_text(encoding="utf-8") if header is None else header
    text = re.sub(r"/\*.*?\*/", " ", text, flags=re.DOTALL)
    text = re.sub(r"//[^\n]*", "", text)
    return _inline_prototypes(_preprocess(text))


def _preprocess(text: str) -> str:
    defined = set()
    active: List[bool] = []
    lines = []
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped.startswith("#"):
            if all(active):
                lines.append(line)
            continue
        directive, _, argument = stripped[1:].strip().partition(" ")
        argument = argument.strip()
        if directive == "ifdef":
            active.append(argument in defined)
        elif directive == "ifndef":
            active.append(argument not in defined)
        elif directive == "else":
            active[-1] = not active[-1]
        elif directive == "endif":
            active.pop()
        elif not all(active) or directive == "include":
            continue
        elif directive == "define":
            name, _, value = argument.partition(" ")
            if value.strip():
                lines.append(f"#define {name} {value.strip()}")
            else:
                defined.add(name)
        else:
            raise ValueError(f"antlr.h has an unsupported directive: {stripped}")
    return "\n".join(lines) + "\n"


def _inline_prototypes(text: str) -> str:
    pieces = []
    position = 0
    for match in re.finditer(r"\bstatic\s+inline\b", text):
        if match.start() < position:
            continue
        body_start = text.index("{", match.end())
        depth = 0
        for index in range(body_start, len(text)):
            if text[index] == "{":
                depth += 1
            elif text[index] == "}":
                depth -= 1
                if depth == 0:
                    break
        else:
            raise ValueError("antlr.h has an unterminated static inline function")
        pieces.append(text[position : match.start()])
        pieces.append(" ".join(text[match.end() : body_start].split()) + ";")
        position = index + 1
    pieces.append(text[position:])
    return "".join(pieces)


def _default_link_args() -> List[str]:
    if sys.platform.startswith("linux"):
        # Keep the symbols of the static library private to the extension
        # module, so that two bundles in one process do not share them.
        return ["-Wl,--exclude-libs,ALL"]
    return []


@dataclass
class StaticLibrary:
    """A Rust static library and the linker arguments that it needs."""

    path: Path
    """The static library (``lib<crate>.a`` or ``<crate>.lib``)."""
    link_args: List[str] = field(default_factory=list)
    """The native libraries of the Rust standard library, as the
    ``native-static-libs`` note of ``rustc`` lists them."""


def build_staticlib(
    package: str,
    *,
    manifest_path: Optional[PathLike] = None,
    features: Iterable[str] = (),
    release: bool = False,
    cargo: str = "cargo",
    cwd: Optional[PathLike] = None,
    env: Optional["dict[str, str]"] = None,
) -> StaticLibrary:
    """Builds the static library of the Cargo package ``package``.

    The package must have the ``staticlib`` crate type. Returns the library
    and the native libraries that the linker must add.
    """
    command = [cargo, "rustc", "--package", package, "--lib", "--message-format=json"]
    if manifest_path is not None:
        command += ["--manifest-path", os.fspath(manifest_path)]
    features = list(features)
    if features:
        command += ["--features", ",".join(features)]
    if release:
        command.append("--release")
    command += ["--", "--print", "native-static-libs"]
    result = subprocess.run(
        command, cwd=cwd, env=env, stdout=subprocess.PIPE, check=True, text=True
    )
    library: Optional[Path] = None
    link_args: List[str] = []
    for line in result.stdout.splitlines():
        if not line.startswith("{"):
            continue
        message = json.loads(line)
        # Only the library of `package` has the staticlib crate type: Cargo
        # builds the dependencies as rlibs.
        if "staticlib" not in message.get("target", {}).get("crate_types", ()):
            continue
        reason = message.get("reason")
        if reason == "compiler-artifact":
            for filename in message["filenames"]:
                if filename.endswith((".a", ".lib")):
                    library = Path(filename)
        elif reason == "compiler-message":
            text = message["message"]["message"]
            if text.startswith("native-static-libs:"):
                link_args = text.split(":", 1)[1].split()
    if library is None:
        raise RuntimeError(f"cargo built no static library for {package}")
    return StaticLibrary(library, link_args)


def ffibuilder(
    module_name: str,
    library: Union[StaticLibrary, PathLike],
    *,
    extra_link_args: Sequence[str] = (),
    **set_source_kwargs: Any,
) -> Any:
    """A ``cffi.FFI`` builder of the extension module ``module_name`` (for
    example ``my_package._native``) that links ``library``.

    Use it in the ``cffi_modules`` of a setuptools build, or call its
    ``compile`` method. ``set_source_kwargs`` go to ``FFI.set_source``.
    """
    import cffi

    if not isinstance(library, StaticLibrary):
        library = StaticLibrary(Path(library))
    # CFFI compiles in a temporary directory, so make all paths absolute.
    library_path = Path(library.path).resolve()
    ffi = cffi.FFI()
    ffi.cdef(cdef())
    ffi.set_source(
        module_name,
        f'#include "{_HEADER}"',
        include_dirs=[str(header_dir()), *set_source_kwargs.pop("include_dirs", [])],
        extra_objects=[str(library_path), *set_source_kwargs.pop("extra_objects", [])],
        extra_link_args=[*_default_link_args(), *library.link_args, *extra_link_args],
        py_limited_api=True,
        **set_source_kwargs,
    )
    return ffi


def compile_bundle(
    module_name: str,
    library: Union[StaticLibrary, PathLike],
    output_dir: PathLike,
    *,
    extra_link_args: Sequence[str] = (),
    verbose: bool = False,
) -> Path:
    """Compiles the extension module ``module_name`` into ``output_dir`` and
    returns its path.

    A dotted ``module_name`` puts the module into the package directory under
    ``output_dir`` (``my_package._native`` gives
    ``output_dir/my_package/_native.abi3.so``).
    """
    ffi = ffibuilder(module_name, library, extra_link_args=extra_link_args)
    output_dir = Path(output_dir).resolve()
    build_dir = output_dir / "build" / module_name
    build_dir.mkdir(parents=True, exist_ok=True)
    with _host_architecture():
        built = Path(ffi.compile(tmpdir=str(build_dir), verbose=verbose))
    if not built.is_absolute():
        built = build_dir / built
    destination = output_dir.joinpath(*module_name.split(".")[:-1], built.name)
    destination.parent.mkdir(parents=True, exist_ok=True)
    os.replace(built, destination)
    return destination


@contextlib.contextmanager
def _host_architecture() -> Iterator[None]:
    """On macOS, builds for the host architecture only.

    A universal2 Python builds extension modules for two architectures, but
    Cargo builds the static library for the host architecture only. An
    ``ARCHFLAGS`` value from the environment has priority.
    """
    if sys.platform != "darwin" or "ARCHFLAGS" in os.environ:
        yield
        return
    os.environ["ARCHFLAGS"] = f"-arch {platform.machine()}"
    try:
        yield
    finally:
        del os.environ["ARCHFLAGS"]
