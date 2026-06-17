"""Fetch a Stats NZ Aotearoa Data Explorer JSON endpoint to Parquet.

The current Stats NZ public data access path is Aotearoa Data Explorer (ADE).
This helper is intentionally endpoint-driven because ADE dataset identifiers and
query URLs are normally created in the ADE interface or API portal.
"""

from __future__ import annotations

import argparse
import logging
import os
from pathlib import Path

import pandas as pd
import requests

USER_AGENT = "open-social-data/0.1 (+https://github.com/open-social-data)"


def fetch_json(endpoint: str, api_key: str | None = None) -> dict:
    headers = {"Accept": "application/json", "User-Agent": USER_AGENT}
    if api_key:
        headers["Ocp-Apim-Subscription-Key"] = api_key
    response = requests.get(endpoint, headers=headers, timeout=90)
    response.raise_for_status()
    return response.json()


def rows_from_payload(payload: dict) -> list[dict]:
    if isinstance(payload.get("value"), list):
        return payload["value"]
    if isinstance(payload.get("data"), list):
        return payload["data"]
    if isinstance(payload.get("observations"), list):
        return payload["observations"]
    raise ValueError("No tabular row list found in JSON payload")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--endpoint", required=True, help="ADE API JSON endpoint")
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument(
        "--api-key-env",
        default="STATS_NZ_API_KEY",
        help="Environment variable containing the ADE subscription key",
    )
    args = parser.parse_args()

    logging.basicConfig(level=logging.INFO, format="%(levelname)s: %(message)s")
    payload = fetch_json(args.endpoint, os.getenv(args.api_key_env))
    rows = rows_from_payload(payload)
    df = pd.DataFrame(rows)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    df.to_parquet(args.output, index=False)
    logging.info("Wrote %s rows to %s", len(df), args.output)


if __name__ == "__main__":
    main()
