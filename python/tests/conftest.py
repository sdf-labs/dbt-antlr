"""Builds the test bundle of `tests/ffi-bundle` into a temporary package.

The package `antlr_test_bundle` holds the generated Python modules of the
bundle (`csv_parser`, `labels_parser`, `keywords_parser`) and the native
extension module `_native`, which links the static library of the bundle.
"""

from __future__ import annotations

import shutil
import sys
from pathlib import Path

import pytest

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO / "python"))

from dbt_antlr import build  # noqa: E402

BUNDLE_CRATE = "dbt-antlr-ffi-test-bundle"
BUNDLE_DIR = REPO / "tests" / "ffi-bundle"
PACKAGE = "antlr_test_bundle"


@pytest.fixture(scope="session", autouse=True)
def bundle_package(tmp_path_factory: pytest.TempPathFactory) -> Path:
    library = build.build_staticlib(
        BUNDLE_CRATE,
        manifest_path=REPO / "Cargo.toml",
        features=["arena-allocation-limit"],
        cwd=REPO,
    )
    output_dir = tmp_path_factory.mktemp("bundle")
    package_dir = output_dir / PACKAGE
    package_dir.mkdir()
    (package_dir / "__init__.py").write_text('"""The FFI test bundle."""\n')
    for module in (BUNDLE_DIR / "python").glob("*_parser.py"):
        shutil.copy(module, package_dir / module.name)
    build.compile_bundle(f"{PACKAGE}._native", library, output_dir)
    sys.path.insert(0, str(output_dir))
    return package_dir


def _module(name: str):
    import importlib

    return importlib.import_module(f"{PACKAGE}.{name}")


@pytest.fixture()
def csv_parser(bundle_package: Path):
    return _module("csv_parser")


@pytest.fixture()
def labels_parser(bundle_package: Path):
    return _module("labels_parser")


@pytest.fixture()
def keywords_parser(bundle_package: Path):
    return _module("keywords_parser")


@pytest.fixture()
def native(bundle_package: Path):
    return _module("_native")
