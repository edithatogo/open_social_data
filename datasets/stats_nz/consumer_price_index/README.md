# Consumers Price Index, New Zealand

**Source:** Stats NZ / Aotearoa Data Explorer
**Original Source Link:** https://www.stats.govt.nz/tools/aotearoa-data-explorer/
**Date Accessed/Processed:** 2026-06-17

## Description

The Consumers Price Index (CPI) measures price change for goods and services bought by households. It is a key dataset for cost-of-living analysis.

## Data Structure & Access

* **Expected source format:** ADE API JSON/SDMX-derived output.
* **Processed format:** Parquet in `data/`.
* **Script:** `scripts/fetch_consumer_price_index.py`
* **Access note:** Set `STATS_NZ_API_KEY` if the ADE endpoint requires an API subscription key.

## Key Information & Variables

* **Data Dictionary:** [`docs/data_dictionary.md`](./docs/data_dictionary.md)
* **Accessible Guide:** [`docs/accessible_guide.md`](./docs/accessible_guide.md)

## Known Issues

No live CPI extract is committed yet. Users should generate the ADE query URL for the specific CPI table they need.

## Last Updated in this Repository

* **Date:** 2026-06-17
* **Changes Made:** Initial dataset documentation and fetch wrapper.
