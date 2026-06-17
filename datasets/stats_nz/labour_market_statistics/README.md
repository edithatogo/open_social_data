# Labour Market Statistics, New Zealand

**Source:** Stats NZ / Aotearoa Data Explorer
**Original Source Link:** https://www.stats.govt.nz/tools/aotearoa-data-explorer/
**Date Accessed/Processed:** 2026-06-17

## Description

Labour market statistics describe employment, unemployment, labour force participation, and related measures. This pack is intended for social and economic labour-market analysis.

## Data Structure & Access

* **Expected source format:** ADE API JSON/SDMX-derived output.
* **Processed format:** Parquet in `data/`.
* **Script:** `scripts/fetch_labour_market_statistics.py`

## Key Information & Variables

* **Data Dictionary:** [`docs/data_dictionary.md`](./docs/data_dictionary.md)
* **Accessible Guide:** [`docs/accessible_guide.md`](./docs/accessible_guide.md)

## Known Issues

No live extract is committed yet. Users should generate an ADE API endpoint for the exact labour-market table and disaggregation required.

## Last Updated in this Repository

* **Date:** 2026-06-17
* **Changes Made:** Initial dataset documentation and fetch wrapper.
