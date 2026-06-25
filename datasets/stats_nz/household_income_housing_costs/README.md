# Household Income and Housing-Cost Statistics, New Zealand

**Source:** Stats NZ / Household Economic Survey and Aotearoa Data Explorer  
**Original Source Link:** https://www.stats.govt.nz/information-releases/household-income-and-housing-cost-statistics-year-ended-june-2025/  
**Date Accessed/Processed:** 2026-06-23

## Description

This pack documents Stats NZ household income and housing-cost statistics. It is a high-priority social dataset because income, housing costs, tenure, and household characteristics are central to cost-of-living and wellbeing analysis.

## Data Structure & Access

* **Expected source format:** ADE API JSON or source release downloadable tables.
* **Processed format:** Parquet in `data/` after a successful fetch.
* **Script:** `scripts/fetch_household_income_housing_costs.py`
* **Access note:** ADE API use may require a Stats NZ API portal subscription key. Set `STATS_NZ_API_KEY` before running.

Example:

```bash
python datasets/stats_nz/household_income_housing_costs/scripts/fetch_household_income_housing_costs.py --endpoint "https://example.stats.govt.nz/ade/household-income-housing-costs.json"
```

## Key Information & Variables

* **Data Dictionary:** [`docs/data_dictionary.md`](./docs/data_dictionary.md)
* **Accessible Guide:** [`docs/accessible_guide.md`](./docs/accessible_guide.md)
* **Source Metadata:** [`source_metadata.json`](./source_metadata.json)

## Methodology & Collection

Stats NZ produces these statistics from the Household Economic Survey. Users should review the official release and methodology notes for survey coverage, sampling error, equivalised income definitions, and changes associated with the transition toward the Household Income and Living Survey.

## Known Issues

This pack does not commit a live ADE response. Exact ADE endpoint URLs and table selections must be captured during a live source refresh.

## License & Usage Terms

Use is subject to Stats NZ reuse terms and the licence specified on the source dataset page.

## Citation

Please cite Stats NZ and the specific household income and housing-cost release or ADE table used.

## Last Updated in this Repository

* **Date:** 2026-06-23
* **Changes Made:** Initial medium-term dataset pack, metadata file, and endpoint-driven fetch wrapper.
