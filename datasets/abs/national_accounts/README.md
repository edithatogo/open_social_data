# Australian National Accounts

**Source:** Australian Bureau of Statistics (ABS)
**Original Source Link:** https://www.abs.gov.au/statistics/economy/national-accounts/australian-national-accounts-national-income-expenditure-and-product
**Date Accessed/Processed:** 2026-06-17

## Description

Australian National Accounts provide measures such as GDP, income, expenditure, production, and related macroeconomic aggregates.

## Data Structure & Access

* **ABS Data API dataflow:** `ANA_AGG`
* **Expected API format:** SDMX-JSON.
* **Processed format:** Parquet in `data/`.
* **Script:** `scripts/fetch_national_accounts.py`

## Key Information & Variables

* **Data Dictionary:** [`docs/data_dictionary.md`](./docs/data_dictionary.md)
* **Accessible Guide:** [`docs/accessible_guide.md`](./docs/accessible_guide.md)

## Known Issues

Live fetch validation is blocked in this environment by DNS resolution for `api.abs.gov.au`. Confirm the current DSD before relying on dimension names.

## Last Updated in this Repository

* **Date:** 2026-06-17
* **Changes Made:** Initial dataset documentation and fetch wrapper.
