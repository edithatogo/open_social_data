# Data Dictionary for Consumer Price Index, Australia

**Dataset Source:** ABS Data API dataflow `CPI`
**Updated:** 2026-06-17

| Variable Name | Description | Type | Notes |
|---|---|---|---|
| `provider` | Provider label. | Text | `abs` when fetched by repository tooling. |
| `dataset_id` | ABS dataflow ID. | Text | `CPI`. |
| `TIME_PERIOD` | Observation period. | Text | Usually quarter for headline CPI. |
| CPI dimension columns | Region, expenditure class, index type, adjustment, or other ABS dimensions. | Text | Exact names come from the ABS DSD. |
| `OBS_VALUE` | Published value. | Text/Numeric | CPI index points or movement, depending on selected measure. |

Use the ABS DSD for authoritative codelists and dimension names.
