# Data Dictionary for Population Estimates, New Zealand

**Dataset Source:** Stats NZ / Aotearoa Data Explorer
**Updated:** 2026-06-17

## Variables

Final column names depend on the ADE query exported. Expected row-level fields include:

| Variable Name | Description | Type | Notes |
|---|---|---|---|
| `provider` | Source provider added by repository tooling when using the Rust provider path. | Text | Usually `stats_nz`. |
| `dataset_id` | Dataset or ADE endpoint identifier. | Text | Should identify the population-estimates query. |
| `TIME_PERIOD` or `Period` | Observation period. | Text | Year, quarter, or other period depending on query. |
| `Region` or geography code | Area represented by the estimate. | Text | Use official Stats NZ geography labels/codes. |
| `Age` | Age or age group. | Text | Present only for age-disaggregated extracts. |
| `Sex` | Sex category. | Text | Present only for sex-disaggregated extracts. |
| `OBS_VALUE` or `Value` | Estimated population. | Numeric/Text | Unit is people unless source metadata says otherwise. |

## Quality Checks

Recommended checks:

* Period is not blank.
* Geography is not blank when a geography dimension is present.
* Values are numeric and non-negative after excluding suppressed or missing observations.

## Further Information

Use the official ADE metadata and dataset notes for codelists, revisions, and geography definitions.
