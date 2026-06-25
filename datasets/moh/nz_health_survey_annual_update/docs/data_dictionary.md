# Data Dictionary for New Zealand Health Survey Annual Update

**Dataset Source:** New Zealand Ministry of Health  
**Date of Dictionary Creation/Update:** 2026-06-23

## Introduction

This dictionary describes the expected fields from Annual Data Explorer CSV exports. Exact column names may vary by export selection, but the concepts below should be retained in processed Parquet files.

## Variables

| Variable Name | Description | Data Type | Allowed Values / Format / Range | Units | Missing Values | Notes / Source Definition |
|---|---|---|---|---|---|---|
| `indicator` | Health survey topic or measure. | String | Source indicator labels | N/A | Blank | Preserve official wording. |
| `population_group` | Sex, age, ethnicity, deprivation, disability, region, or other group. | String | Source categories | N/A | Blank | May be split into multiple columns by export. |
| `survey_year` | Survey year or pooled period. | String | Year labels such as `2024/25` | N/A | Blank | Use source period labels. |
| `estimate` | Reported percentage, mean, or other estimate. | Numeric | Source-provided values | Percent or source unit | Suppressed/blank | Check `unit` before comparison. |
| `lower_confidence_interval` | Lower bound if supplied. | Numeric | Source-provided values | Same as estimate | Blank | Use for uncertainty-aware interpretation. |
| `upper_confidence_interval` | Upper bound if supplied. | Numeric | Source-provided values | Same as estimate | Blank | Use for uncertainty-aware interpretation. |
| `unit` | Measurement unit. | String | Percent, count, rate, mean | N/A | Blank | Required when exports contain multiple measure types. |
| `flag` | Suppression, reliability, or note flag. | String | Source flags | N/A | Blank | Treat flagged values carefully. |

## Code Lists / Classifications

Expected codelists include survey year, indicator, population group, age group, gender/sex, ethnicity, neighbourhood deprivation, disability status, and health region.

## Further Information

* Source publication: https://www.health.govt.nz/publications/annual-update-of-key-results-202425-new-zealand-health-survey
* Annual Data Explorer: linked from the source publication page.
