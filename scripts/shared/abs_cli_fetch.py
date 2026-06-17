"""Fetch an ABS dataflow through the Rust CLI and write Parquet output."""

from __future__ import annotations

import argparse
import logging
import subprocess
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dataflow", required=True, help="ABS Data API dataflow ID")
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument(
        "--quality-report",
        type=Path,
        default=None,
        help="Optional JSON quality report path",
    )
    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")
    args.output.parent.mkdir(parents=True, exist_ok=True)
    command = [
        "cargo",
        "run",
        "--bin",
        "open-social-data-cli",
        "--",
        "fetch",
        "abs",
        args.dataflow,
        "--output",
        str(args.output),
    ]
    if args.quality_report:
        args.quality_report.parent.mkdir(parents=True, exist_ok=True)
        command.extend(["--quality-report", str(args.quality_report)])
    logging.info("Running %s", " ".join(command))
    subprocess.run(command, check=True)


if __name__ == "__main__":
    main()
