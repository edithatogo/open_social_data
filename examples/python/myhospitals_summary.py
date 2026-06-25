"""Summarise local AIHW MyHospitals Parquet extracts."""

from __future__ import annotations

import argparse
from pathlib import Path

import pandas as pd


ROOT = Path(__file__).resolve().parents[2]
DATA_DIR = ROOT / "datasets" / "aihw" / "myhospitals" / "data"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--data-dir", type=Path, default=DATA_DIR)
    parser.add_argument("--limit", type=int, default=10)
    args = parser.parse_args()

    rows: list[dict[str, object]] = []
    for path in sorted(args.data_dir.glob("*.parquet")):
        df = pd.read_parquet(path, columns=["measure_category_code", "measure_name"])
        rows.append(
            {
                "file": path.name,
                "rows": len(df),
                "measure_category_code": ", ".join(sorted(df["measure_category_code"].dropna().unique())[:3]),
                "measure_count": df["measure_name"].nunique(dropna=True),
            }
        )
    if not rows:
        raise SystemExit(f"No Parquet files found in {args.data_dir}")
    summary = pd.DataFrame(rows).sort_values(["measure_category_code", "file"])
    print(summary.head(args.limit).to_string(index=False))


if __name__ == "__main__":
    main()
