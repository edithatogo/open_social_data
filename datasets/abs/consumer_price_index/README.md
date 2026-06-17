# Consumer Price Index, Australia

**Source:** Australian Bureau of Statistics (ABS)
**Original Source Link:** https://www.abs.gov.au/statistics/economy/price-indexes-and-inflation/consumer-price-index-australia
**Date Accessed/Processed:** 2026-06-17

## Description

The Consumer Price Index (CPI) measures price changes for a basket of goods and services bought by Australian households. It is a key inflation dataset.

## Data Structure & Access

* **ABS Data API dataflow:** `CPI`
* **Expected API format:** SDMX-JSON.
* **Processed format:** Parquet in `data/`.
* **Script:** `scripts/fetch_consumer_price_index.py`

## Key Information & Variables

* **Data Dictionary:** [`docs/data_dictionary.md`](./docs/data_dictionary.md)
* **Accessible Guide:** [`docs/accessible_guide.md`](./docs/accessible_guide.md)

## Known Issues

This environment currently cannot resolve `api.abs.gov.au`, so live fetch validation was not run here. The ABS Data API user guide also notes the newer `data.api.abs.gov.au/rest` path; provider migration remains a follow-up if the old endpoint is unavailable.

## Last Updated in this Repository

* **Date:** 2026-06-17
* **Changes Made:** Initial dataset documentation and fetch wrapper.
