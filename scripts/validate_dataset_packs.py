"""Validate required documentation and access scripts for dataset packs."""

from __future__ import annotations

import argparse
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
REQUIRED_SOURCES = ("abs", "stats_nz", "aihw", "moh")


def dataset_dirs(dataset_root: Path) -> list[Path]:
    dirs: list[Path] = []
    for source in REQUIRED_SOURCES:
        source_dir = dataset_root / source
        if not source_dir.exists():
            continue
        dirs.extend(
            path
            for path in sorted(source_dir.iterdir())
            if path.is_dir() and not path.name.startswith(".")
        )
    return dirs


def has_match(base: Path, pattern: str) -> bool:
    return any(base.glob(pattern))


def validate_dataset(path: Path) -> list[str]:
    issues: list[str] = []
    if not (path / "README.md").is_file():
        issues.append("missing README.md")
    if not (path / "SESSION_LOG.md").is_file():
        issues.append("missing SESSION_LOG.md")
    docs = path / "docs"
    if not docs.is_dir():
        issues.append("missing docs/")
    else:
        if not has_match(docs, "data_dictionary*.md"):
            issues.append("missing docs/data_dictionary*.md")
        if not has_match(docs, "accessible_guide*.md"):
            issues.append("missing docs/accessible_guide*.md")
    scripts = path / "scripts"
    if not scripts.is_dir() or not has_match(scripts, "*.py"):
        issues.append("missing scripts/*.py access script")
    return issues


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--root",
        type=Path,
        default=ROOT / "datasets",
        help="Dataset root to validate. Defaults to repository datasets/.",
    )
    args = parser.parse_args()

    failed = False
    for dataset in dataset_dirs(args.root):
        issues = validate_dataset(dataset)
        rel = dataset.relative_to(ROOT)
        if issues:
            failed = True
            print(f"FAIL {rel}: {'; '.join(issues)}")
        else:
            print(f"OK {rel}")

    if failed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
