import argparse
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
OUTPUT = Path(__file__).resolve().parents[1] / "data" / "nz_health_survey_annual_update.parquet"

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--url", required=True, help="Annual Data Explorer CSV URL or local CSV path")
    args = parser.parse_args()
    subprocess.run(
        [
            sys.executable,
            str(ROOT / "scripts" / "shared" / "csv_to_parquet.py"),
            "--url",
            args.url,
            "--output",
            str(OUTPUT),
        ],
        check=True,
    )


if __name__ == "__main__":
    main()
