# Average Weekly Earnings, Australia

**Source:** Australian Bureau of Statistics (ABS)  
**Original Source Link:** https://www.abs.gov.au/statistics/application-programming-interfaces-apis/indicator-api  
**Date Accessed/Processed:** 2026-06-23

## Description

Average Weekly Earnings reports earnings measures by sex, sector, state or territory, and other published dimensions. It is useful for wage, labour-market, and cost-of-living analysis.

## Data Structure & Access

* **ABS API coverage:** ABS Indicator API lists Average Weekly Earnings among published content.
* **Candidate ABS Data API dataflow:** `AWE` pending live dataflow confirmation.
* **Expected API format:** SDMX-JSON through the repository ABS provider path.
* **Processed format:** Parquet in `data/`.
* **Script:** `scripts/fetch_average_weekly_earnings.py`

## Key Information & Variables

* **Data Dictionary:** [`docs/data_dictionary.md`](./docs/data_dictionary.md)
* **Accessible Guide:** [`docs/accessible_guide.md`](./docs/accessible_guide.md)
* **Source Metadata:** [`source_metadata.json`](./source_metadata.json)

## Known Issues

The repository wrapper uses the CLI-backed ABS provider path. Live validation of the exact `AWE` dataflow and codelists should be completed when `api.abs.gov.au` access is available.

## Last Updated in this Repository

* **Date:** 2026-06-23
* **Changes Made:** Initial medium-term dataset pack, metadata file, and CLI-backed fetch wrapper.
