from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
OUTPUT = Path(__file__).resolve().parents[1] / "data" / "national_accounts.parquet"
QUALITY = Path(__file__).resolve().parents[1] / "logs" / "national_accounts_quality.json"

if __name__ == "__main__":
    subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "shared" / "abs_cli_fetch.py"), "--dataflow", "ANA_AGG", "--output", str(OUTPUT), "--quality-report", str(QUALITY)],
        check=True,
    )
