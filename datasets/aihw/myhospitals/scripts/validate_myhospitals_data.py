"""Validate local AIHW MyHospitals Parquet extracts."""

from __future__ import annotations

import argparse
from pathlib import Path

import pandas as pd

DATASET_DIR = Path(__file__).resolve().parent.parent
DATA_DIR = DATASET_DIR / "data"
REQUIRED_COLUMNS = {
    "measure_category_code",
    "measure_name",
    "reported_measure_name",
    "reporting_unit_name",
    "data_period",
    "formatted_value",
}


def validate_file(path: Path) -> list[str]:
    df = pd.read_parquet(path)
    issues: list[str] = []
    missing = sorted(REQUIRED_COLUMNS.difference(df.columns))
    if missing:
        issues.append(f"missing required columns: {', '.join(missing)}")
    if df.empty:
        issues.append("file has zero rows")
    if "raw_value" in df.columns:
        numeric = pd.to_numeric(df["raw_value"], errors="coerce")
        published = df["formatted_value"].astype(str).str.strip().ne("")
        if published.any() and numeric[published].isna().all():
            issues.append("raw_value has no numeric values for published formatted values")
    return issues


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--data-dir", type=Path, default=DATA_DIR)
    args = parser.parse_args()

    files = sorted(args.data_dir.glob("*.parquet"))
    if not files:
        raise SystemExit(f"No Parquet files found in {args.data_dir}")

    failed = False
    for path in files:
        issues = validate_file(path)
        if issues:
            failed = True
            print(f"FAIL {path.name}: {'; '.join(issues)}")
        else:
            df = pd.read_parquet(path)
            print(f"OK {path.name}: {len(df)} rows, {len(df.columns)} columns")

    if failed:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
