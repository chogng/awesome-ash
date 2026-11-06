"""Stage declarative and Rust extension resources for shared packages."""

import json
import re
import stat
from pathlib import Path

from build.lib.tree import copy_regular_tree


def copy_builtin_extensions(source_root: Path, destination: Path) -> None:
    destination.mkdir(parents=True)
    excluded = (
        "BUILD.bazel",
        "Cargo.lock",
        "Cargo.toml",
        "browser",
        "build.rs",
        "common",
        "node_modules",
        "src",
        "target",
        "test",
        "tests",
    )
    for package in sorted((source_root / "extensions").iterdir()):
        manifest = package / "package.json"
        if not manifest.is_file():
            continue
        if not re.fullmatch(r"[A-Za-z0-9_-]+", package.name) or not stat.S_ISREG(
            manifest.lstat().st_mode
        ):
            raise RuntimeError(f"Invalid built-in extension package: {package}")
        metadata = json.loads(manifest.read_text(encoding="utf-8"))
        if metadata.get("ashDistribution") == "marketplace" or metadata.get(
            "ashRuntime", {}
        ).get("client"):
            continue
        copy_regular_tree(
            package, destination / package.name, "extension", excluded_names=excluded
        )


def validate_builtin_extensions(extensions_directory: Path) -> None:
    if extensions_directory.is_symlink() or not extensions_directory.is_dir():
        raise RuntimeError("Package is missing built-in extensions")
    extension_directories = sorted(
        extensions_directory.iterdir(), key=lambda path: path.name
    )
    if not extension_directories:
        raise RuntimeError("Package contains no built-in extensions")
    for extension_directory in extension_directories:
        if (
            extension_directory.is_symlink()
            or not extension_directory.is_dir()
            or not (extension_directory / "package.json").is_file()
        ):
            raise RuntimeError(
                "Package contains an invalid built-in extension: {}".format(
                    extension_directory
                )
            )
