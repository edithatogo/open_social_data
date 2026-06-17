"""Build a small static dashboard from local AIHW MyHospitals Parquet files."""

from __future__ import annotations

from html import escape
from pathlib import Path

import pandas as pd


ROOT = Path(__file__).resolve().parents[1]
DATA_DIR = ROOT / "datasets" / "aihw" / "myhospitals" / "data"
OUTPUT = ROOT / "docs" / "advanced" / "myhospitals_summary.html"


def load_frames() -> pd.DataFrame:
    files = sorted(DATA_DIR.glob("*.parquet"))
    if not files:
        raise SystemExit(f"No Parquet files found in {DATA_DIR}")
    frames = [
        pd.read_parquet(path).dropna(axis=1, how="all").assign(source_file=path.name)
        for path in files
    ]
    return pd.concat(frames, ignore_index=True, sort=False)


def category_summary(df: pd.DataFrame) -> pd.DataFrame:
    columns = ["measure_category_code", "source_file"]
    available = [column for column in columns if column in df.columns]
    summary = (
        df.groupby(available, dropna=False)
        .size()
        .reset_index(name="rows")
        .sort_values(["measure_category_code", "rows"], ascending=[True, False])
    )
    return summary


def top_units(df: pd.DataFrame, limit: int = 12) -> pd.DataFrame:
    columns = ["measure_category_code", "reporting_unit_name"]
    if not all(column in df.columns for column in columns):
        return pd.DataFrame(columns=columns + ["rows"])
    return (
        df.groupby(columns, dropna=False)
        .size()
        .reset_index(name="rows")
        .sort_values("rows", ascending=False)
        .head(limit)
    )


def table_html(frame: pd.DataFrame) -> str:
    header = "".join(f"<th>{escape(str(column))}</th>" for column in frame.columns)
    rows = []
    for _, row in frame.iterrows():
        cells = "".join(f"<td>{escape(str(value))}</td>" for value in row)
        rows.append(f"<tr>{cells}</tr>")
    return f"<table><thead><tr>{header}</tr></thead><tbody>{''.join(rows)}</tbody></table>"


def main() -> None:
    df = load_frames()
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)
    categories = category_summary(df)
    units = top_units(df)
    generated = pd.Timestamp.utcnow().strftime("%Y-%m-%d %H:%M UTC")
    html = f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>AIHW MyHospitals Local Summary</title>
  <style>
    body {{ font-family: Arial, sans-serif; margin: 2rem; color: #1f2933; }}
    h1, h2 {{ color: #102a43; }}
    table {{ border-collapse: collapse; margin: 1rem 0 2rem; width: 100%; }}
    th, td {{ border: 1px solid #bcccdc; padding: 0.5rem; text-align: left; }}
    th {{ background: #f0f4f8; }}
    .metric {{ display: inline-block; margin-right: 2rem; font-size: 1.1rem; }}
  </style>
</head>
<body>
  <h1>AIHW MyHospitals Local Summary</h1>
  <p>Generated from local Parquet extracts at {escape(generated)}. This is a static prototype for advanced access review.</p>
  <p class="metric"><strong>Total rows:</strong> {len(df):,}</p>
  <p class="metric"><strong>Total columns:</strong> {len(df.columns):,}</p>
  <h2>Rows by Measure Category and Source File</h2>
  {table_html(categories)}
  <h2>Largest Reporting Unit Groups</h2>
  {table_html(units)}
</body>
</html>
"""
    OUTPUT.write_text(html, encoding="utf-8")
    print(f"Wrote {OUTPUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
