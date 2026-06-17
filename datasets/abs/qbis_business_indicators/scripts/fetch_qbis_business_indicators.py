from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
OUTPUT = Path(__file__).resolve().parents[1] / "data" / "qbis_business_indicators.parquet"
QUALITY = Path(__file__).resolve().parents[1] / "logs" / "qbis_business_indicators_quality.json"

if __name__ == "__main__":
    subprocess.run(
        [
            sys.executable,
            str(ROOT / "scripts" / "shared" / "abs_cli_fetch.py"),
            "--dataflow",
            "QBIS",
            "--output",
            str(OUTPUT),
            "--quality-report",
            str(QUALITY),
        ],
        check=True,
    )
