# Data Dictionary for Australian National Accounts

**Dataset Source:** ABS Data API dataflow `ANA_AGG`
**Updated:** 2026-06-17

| Variable Name | Description | Type | Notes |
|---|---|---|---|
| `provider` | Provider label. | Text | `abs`. |
| `dataset_id` | ABS dataflow ID. | Text | `ANA_AGG`. |
| `TIME_PERIOD` | Observation period. | Text | Quarter or year depending on selected series. |
| Account/aggregate dimensions | National accounts item, price basis, adjustment, sector, or other ABS dimensions. | Text | Exact columns come from the ABS DSD. |
| `OBS_VALUE` | Published value. | Text/Numeric | Often AUD millions, index points, or percent change. |

Use source metadata to distinguish current prices, chain volume measures, trend, seasonally adjusted, and original series.
