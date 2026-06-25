# Data Dictionary for Household Income and Housing-Cost Statistics

**Dataset Source:** Stats NZ household income and housing-cost statistics  
**Date of Dictionary Creation/Update:** 2026-06-23

## Introduction

This dictionary describes the repository's expected tabular representation of Stats NZ household income and housing-cost data. Column names can vary by ADE table, so the variables below identify the standard concepts to preserve when a live endpoint is selected.

## Variables

| Variable Name | Description | Data Type | Allowed Values / Format / Range | Units | Missing Values | Notes / Source Definition |
|---|---|---|---|---|---|---|
| `period` | Survey year or year ended June reference period. | String | Year or year-ended label | N/A | Blank | Use source period labels where possible. |
| `measure` | Income, housing-cost, tenure, or related statistic name. | String | Source table labels | N/A | Blank | Preserve official Stats NZ wording. |
| `population_group` | Household, person, family, ethnicity, age, tenure, or other group. | String | Source categories | N/A | Blank | Can combine with other classification columns in wide tables. |
| `geography` | Geographic area if supplied. | String | NZ, region, territorial authority, or other source geography | N/A | Blank | Record official area labels and codes when available. |
| `value` | Numeric estimate. | Numeric | Source-provided number | Dollars, percent, count, or ratio | Suppressed/blank | Check `unit` before comparing values. |
| `unit` | Measurement unit. | String | NZD, percent, count, index, ratio | N/A | Blank | Required for mixed-measure tables. |
| `status` | Provisional, revised, suppressed, or quality flag if supplied. | String | Source flags | N/A | Blank | Treat suppression and high sampling error as caveats. |

## Code Lists / Classifications

| Concept | Expected Source Classification |
|---|---|
| Geography | Stats NZ geographic classifications used by the selected ADE table. |
| Household type | Official household or family type categories from the release table. |
| Tenure | Owner, renter, and other tenure categories where provided. |
| Income concept | Source definitions such as total income, disposable income, equivalised income, and housing-cost ratios. |

## Further Information

* Source release: https://www.stats.govt.nz/information-releases/household-income-and-housing-cost-statistics-year-ended-june-2025/
* Method note: https://www.stats.govt.nz/methods/about-the-transition-from-the-household-economic-survey-to-the-household-income-and-living-survey/
