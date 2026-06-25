import argparse
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
OUTPUT = Path(__file__).resolve().parents[1] / "data" / "household_income_housing_costs.parquet"

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--endpoint", required=True, help="ADE API JSON endpoint")
    args = parser.parse_args()
    subprocess.run(
        [
            sys.executable,
            str(ROOT / "scripts" / "shared" / "stats_nz_ade_fetcher.py"),
            "--endpoint",
            args.endpoint,
            "--output",
            str(OUTPUT),
        ],
        check=True,
    )


if __name__ == "__main__":
    main()
