"""Small example queries over local AIHW MyHospitals Parquet files."""

from __future__ import annotations

import argparse
from pathlib import Path

import pandas as pd

DATASET_DIR = Path(__file__).resolve().parent.parent
DATA_DIR = DATASET_DIR / "data"


def latest_rows(df: pd.DataFrame) -> pd.DataFrame:
    if "reporting_end_date" not in df.columns:
        return df
    dates = pd.to_datetime(df["reporting_end_date"], errors="coerce")
    if dates.notna().any():
        return df.loc[dates == dates.max()]
    return df


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--data-dir", type=Path, default=DATA_DIR)
    parser.add_argument("--limit", type=int, default=10)
    args = parser.parse_args()

    files = sorted(args.data_dir.glob("*.parquet"))
    if not files:
        raise SystemExit(f"No Parquet files found in {args.data_dir}")

    frames = [
        frame.dropna(axis=1, how="all")
        for frame in (pd.read_parquet(path) for path in files)
        if not frame.empty
    ]
    if not frames:
        raise SystemExit(f"No non-empty Parquet files found in {args.data_dir}")
    df = pd.concat(frames, ignore_index=True, sort=False)
    latest = latest_rows(df)
    columns = [
        column
        for column in [
            "measure_category_code",
            "reporting_unit_name",
            "measure_name",
            "reported_measure_name",
            "data_period",
            "formatted_value",
        ]
        if column in latest.columns
    ]
    print(latest[columns].head(args.limit).to_string(index=False))


if __name__ == "__main__":
    main()
