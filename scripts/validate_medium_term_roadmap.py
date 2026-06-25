"""Validate Track 10 medium-term roadmap artefacts."""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATASETS = ROOT / "datasets"
GUIDES = ROOT / "docs" / "guides"
TRACK = ROOT / "conductor" / "tracks" / "medium_term_expansion_20260618"
REQUIRED_GUIDES = {
    "understanding-social-statistics-concepts.md",
    "interpreting-common-visualizations.md",
    "ethical-use-of-social-data.md",
}


def dataset_dirs() -> list[Path]:
    return [
        path
        for source in sorted(DATASETS.iterdir())
        if source.is_dir()
        for path in sorted(source.iterdir())
        if path.is_dir()
    ]


def has_pack_shape(path: Path) -> bool:
    docs = path / "docs"
    scripts = path / "scripts"
    return (
        (path / "README.md").is_file()
        and (path / "SESSION_LOG.md").is_file()
        and docs.is_dir()
        and any(docs.glob("data_dictionary*.md"))
        and any(docs.glob("accessible_guide*.md"))
        and scripts.is_dir()
        and any(scripts.glob("*.py"))
    )



def run(command: list[str], env: dict[str, str] | None = None) -> tuple[bool, str]:
    completed = subprocess.run(
        command,
        cwd=ROOT,
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    output = completed.stdout.strip().splitlines()
    return completed.returncode == 0, output[-1] if output else "no output"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--run-examples", action="store_true")
    args = parser.parse_args()

    failures: list[str] = []
    candidates = TRACK / "dataset_candidates.md"
    if not candidates.is_file():
        failures.append("missing dataset candidate backlog")

    packs = [path for path in dataset_dirs() if has_pack_shape(path)]
    new_source_packs = [path for path in packs if path.parent.name not in {"abs", "stats_nz", "aihw"}]
    metadata_packs = [path for path in dataset_dirs() if (path / "source_metadata.json").is_file()]

    if len(packs) < 11:
        failures.append(f"expected at least 11 shaped packs after Track 10, found {len(packs)}")
    if not new_source_packs:
        failures.append("expected at least one non-ABS/Stats NZ/AIHW dataset pack")
    missing_guides = sorted(name for name in REQUIRED_GUIDES if not (GUIDES / name).is_file())
    if missing_guides:
        failures.append(f"missing guides: {', '.join(missing_guides)}")
    if len(metadata_packs) < 3:
        failures.append(f"expected at least three source metadata files, found {len(metadata_packs)}")

    ok, detail = run([sys.executable, "scripts/validate_source_metadata.py"])
    if not ok:
        failures.append(f"source metadata validation failed: {detail}")

    if args.run_examples:
        rust_env = os.environ.copy()
        rust_env.setdefault("CARGO_BUILD_JOBS", "1")
        rust_env.setdefault("CARGO_PROFILE_DEV_DEBUG", "0")
        for command in [
            ["cargo", "run", "--bin", "open-social-data-cli", "--", "examples", "myhospitals-summary", "--limit", "5"],
            ["cargo", "run", "--bin", "open-social-data-cli", "--", "examples", "source-metadata-inventory"],
        ]:
            ok, detail = run(command, env=rust_env)
            if not ok:
                failures.append(f"Rust CLI example failed {' '.join(command)}: {detail}")

    if failures:
        for failure in failures:
            print(f"FAIL {failure}")
        raise SystemExit(1)
    print("OK medium-term roadmap artefacts")


if __name__ == "__main__":
    main()
