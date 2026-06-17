# Labour Force, Australia

**Source:** Australian Bureau of Statistics (ABS)
**Original Source Link:** https://www.abs.gov.au/statistics/labour/employment-and-unemployment/labour-force-australia
**Date Accessed/Processed:** 2026-06-17

## Description

Labour Force, Australia reports employment, unemployment, participation, hours worked, and related labour-market indicators.

## Data Structure & Access

* **ABS Data API dataflow:** `LF`
* **Expected API format:** SDMX-JSON.
* **Processed format:** Parquet in `data/`.
* **Script:** `scripts/fetch_labour_force.py`

## Key Information & Variables

* **Data Dictionary:** [`docs/data_dictionary.md`](./docs/data_dictionary.md)
* **Accessible Guide:** [`docs/accessible_guide.md`](./docs/accessible_guide.md)

## Known Issues

Live fetch validation is blocked in this environment by DNS resolution for `api.abs.gov.au`.

## Last Updated in this Repository

* **Date:** 2026-06-17
* **Changes Made:** Initial dataset documentation and fetch wrapper.
