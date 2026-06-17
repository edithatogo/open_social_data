"""Fetch AIHW MyHospitals flat formatted data extracts.

Examples:
    python datasets/aihw/myhospitals/scripts/fetch_aihw_myhospitals_data.py --category MYH-ED-WAITS
    python datasets/aihw/myhospitals/scripts/fetch_aihw_myhospitals_data.py --category MYH-HH --max-pages 2
"""

from __future__ import annotations

import argparse
import json
import logging
from datetime import datetime
from pathlib import Path

import pandas as pd
import requests

BASE_URL = "https://myhospitalsapi.aihw.gov.au/api/v1"
USER_AGENT = (
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
    "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
)
DEFAULT_CATEGORIES = [
    "MYH-ED-WAITS",
    "MYH-ADM",
    "MYH-ES",
    "MYH-CANCER",
    "MYH-LOS",
    "MYH-HH",
]

SCRIPT_DIR = Path(__file__).resolve().parent
DATASET_DIR = SCRIPT_DIR.parent
DATA_DIR = DATASET_DIR / "data"
LOG_DIR = DATASET_DIR / "logs"
LOG_FILE = LOG_DIR / "myhospitals_processing.log"


def configure_logging() -> None:
    LOG_DIR.mkdir(parents=True, exist_ok=True)
    logging.basicConfig(
        level=logging.INFO,
        format="%(asctime)s - %(levelname)s - %(message)s",
        handlers=[logging.FileHandler(LOG_FILE), logging.StreamHandler()],
    )


def fetch_flat_formatted_data(
    measure_category_code: str,
    skip: int = 0,
    top: int = 1000,
) -> dict | None:
    """Fetch one page of flat formatted data for a measure category."""
    url = f"{BASE_URL}/flat-formatted-data-extract/{measure_category_code}"
    headers = {"Accept": "application/json", "User-Agent": USER_AGENT}
    params = {"skip": skip, "top": top}

    logging.info("Fetching %s with params %s", url, params)
    try:
        response = requests.get(url, headers=headers, params=params, timeout=90)
        response.raise_for_status()
        return response.json()
    except requests.exceptions.HTTPError as error:
        logging.error("HTTP error: %s", error)
        logging.error(
            "Response content: %s",
            response.content.decode(errors="ignore") if response.content else "No content",
        )
    except requests.exceptions.RequestException as error:
        logging.error("Request error: %s", error)
    except json.JSONDecodeError:
        logging.error("Failed to decode JSON response")
        logging.error("Response content: %s", response.text)
    return None


def result_rows(api_response: dict) -> list[dict]:
    result = api_response.get("result", {})
    rows = result.get("data")
    if not isinstance(rows, list):
        raise ValueError("API response missing result.data list")
    return rows


def process_and_save_data(
    measure_category_code: str,
    output_dir: Path = DATA_DIR,
    filename_prefix: str = "aihw_myhospitals",
    top: int = 1000,
    max_pages: int | None = None,
) -> Path | None:
    """Fetch paginated data, convert it to a DataFrame, and save Parquet."""
    all_rows: list[dict] = []
    skip = 0
    page_num = 1

    while True:
        if max_pages is not None and page_num > max_pages:
            logging.warning("Reached max page limit of %s for %s", max_pages, measure_category_code)
            break

        api_response = fetch_flat_formatted_data(measure_category_code, skip=skip, top=top)
        if api_response is None:
            raise RuntimeError(f"Failed to fetch page {page_num} for {measure_category_code}")

        rows = result_rows(api_response)
        if not rows:
            logging.info("No rows on page %s; stopping", page_num)
            break

        all_rows.extend(rows)
        logging.info(
            "Page %s: fetched %s rows; total rows %s",
            page_num,
            len(rows),
            len(all_rows),
        )

        if len(rows) < top:
            logging.info("Short page returned; assuming final page")
            break

        skip += top
        page_num += 1

    if not all_rows:
        logging.info("No rows fetched for %s", measure_category_code)
        return None

    df = pd.DataFrame(all_rows)
    output_dir.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    output_path = output_dir / f"{filename_prefix}_{measure_category_code}_{timestamp}.parquet"
    df.to_parquet(output_path, index=False)
    logging.info("Wrote %s rows and %s columns to %s", len(df), len(df.columns), output_path)
    return output_path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--category",
        action="append",
        choices=DEFAULT_CATEGORIES,
        help="Measure category to fetch. Repeat for multiple categories. Defaults to all configured categories.",
    )
    parser.add_argument("--output-dir", type=Path, default=DATA_DIR)
    parser.add_argument("--top", type=int, default=1000)
    parser.add_argument(
        "--max-pages",
        type=int,
        default=None,
        help="Optional page limit for samples or cautious test runs.",
    )
    return parser.parse_args()


def main() -> None:
    configure_logging()
    args = parse_args()
    categories = args.category or DEFAULT_CATEGORIES
    logging.info("Starting MyHospitals fetch for categories: %s", ", ".join(categories))
    for category in categories:
        process_and_save_data(
            category,
            output_dir=args.output_dir,
            top=args.top,
            max_pages=args.max_pages,
        )
    logging.info("Finished MyHospitals fetch")


if __name__ == "__main__":
    main()
