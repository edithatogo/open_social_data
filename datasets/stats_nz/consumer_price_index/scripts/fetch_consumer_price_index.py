from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[4]
OUTPUT = Path(__file__).resolve().parents[1] / "data" / "consumer_price_index.parquet"

if __name__ == "__main__":
    endpoint = sys.argv[sys.argv.index("--endpoint") + 1] if "--endpoint" in sys.argv else None
    if not endpoint:
        raise SystemExit("Usage: python fetch_consumer_price_index.py --endpoint <ADE JSON endpoint>")
    subprocess.run(
        [sys.executable, str(ROOT / "scripts" / "shared" / "stats_nz_ade_fetcher.py"), "--endpoint", endpoint, "--output", str(OUTPUT)],
        check=True,
    )
