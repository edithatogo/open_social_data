"""Fetch a CSV resource and write it to Parquet."""

from __future__ import annotations

import argparse
import logging
from pathlib import Path

import pandas as pd


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--url", required=True, help="CSV URL or local CSV path")
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")
    df = pd.read_csv(args.url)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    df.to_parquet(args.output, index=False)
    logging.info("Wrote %s rows to %s", len(df), args.output)


if __name__ == "__main__":
    main()
