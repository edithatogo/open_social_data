"""Validate source metadata files for dataset packs."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
REQUIRED_FIELDS = {
    "source_agency",
    "source_title",
    "source_url",
    "access_method",
    "licence",
    "update_cadence",
    "methodology_url",
    "units",
    "codelists",
    "caveats",
    "official_metadata_status",
}


def dataset_dirs(dataset_root: Path) -> list[Path]:
    return [
        path
        for source in sorted(dataset_root.iterdir())
        if source.is_dir()
        for path in sorted(source.iterdir())
        if path.is_dir() and not path.name.startswith(".")
    ]


def validate_metadata(path: Path) -> list[str]:
    issues: list[str] = []
    metadata_path = path / "source_metadata.json"
    if not metadata_path.is_file():
        issues.append("missing source_metadata.json")
        return issues
    try:
        data = json.loads(metadata_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as error:
        return [f"invalid JSON: {error}"]

    missing = sorted(REQUIRED_FIELDS.difference(data))
    if missing:
        issues.append(f"missing fields: {', '.join(missing)}")
    for field in ("units", "codelists", "caveats"):
        if field in data and (not isinstance(data[field], list) or not data[field]):
            issues.append(f"{field} must be a non-empty list")
    return issues


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, default=ROOT / "datasets")
    parser.add_argument(
        "--require-all",
        action="store_true",
        help="Require source_metadata.json for every dataset pack, not only packs that have it.",
    )
    args = parser.parse_args()

    failed = False
    checked = 0
    for dataset in dataset_dirs(args.root):
        has_metadata = (dataset / "source_metadata.json").is_file()
        if not has_metadata and not args.require_all:
            continue
        checked += 1
        issues = validate_metadata(dataset)
        rel = dataset.relative_to(ROOT)
        if issues:
            failed = True
            print(f"FAIL {rel}: {'; '.join(issues)}")
        else:
            print(f"OK {rel}")
    if checked == 0:
        failed = True
        print("FAIL metadata: no source_metadata.json files found")
    if failed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
