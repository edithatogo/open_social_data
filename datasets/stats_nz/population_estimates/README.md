# Population Estimates, New Zealand

**Source:** Stats NZ / Aotearoa Data Explorer
**Original Source Link:** https://www.stats.govt.nz/tools/aotearoa-data-explorer/
**Date Accessed/Processed:** 2026-06-17

## Description

Population estimates are core demographic series used to understand how many people live in New Zealand and how that changes by time, geography, age, and sex. This pack prioritises population estimates because they are a baseline denominator for social indicators.

## Data Structure & Access

* **Expected source format:** ADE API JSON/SDMX-derived output.
* **Processed format:** Parquet in `data/` after a successful fetch.
* **Script:** `scripts/fetch_population_estimates.py`
* **Access note:** ADE API use may require a Stats NZ API portal subscription key. Set `STATS_NZ_API_KEY` before running.

Example:

```bash
python datasets/stats_nz/population_estimates/scripts/fetch_population_estimates.py --endpoint "https://example.stats.govt.nz/ade/population-estimates.json"
```

## Key Information & Variables

* **Data Dictionary:** [`docs/data_dictionary.md`](./docs/data_dictionary.md)
* **Accessible Guide:** [`docs/accessible_guide.md`](./docs/accessible_guide.md)

## Methodology & Collection

Stats NZ publishes population estimates through Aotearoa Data Explorer. Users should review the official dataset notes for population concept, geography, resident population basis, and revision status before analysis.

## Known Issues

This repository does not commit a live ADE API response for this dataset yet. The pack contains documentation and a configured fetch path.

## License & Usage Terms

Use is subject to Stats NZ terms and the licence specified on the source dataset page.

## Citation

Please cite Stats NZ and the specific Aotearoa Data Explorer dataset/version used.

## Last Updated in this Repository

* **Date:** 2026-06-17
* **Changes Made:** Initial dataset documentation and fetch wrapper.
