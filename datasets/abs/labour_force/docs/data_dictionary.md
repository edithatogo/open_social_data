# Data Dictionary for Labour Force, Australia

**Dataset Source:** ABS Data API dataflow `LF`
**Updated:** 2026-06-17

| Variable Name | Description | Type | Notes |
|---|---|---|---|
| `provider` | Provider label. | Text | `abs`. |
| `dataset_id` | ABS dataflow ID. | Text | `LF`. |
| `TIME_PERIOD` | Observation period. | Text | Usually month. |
| Labour-force dimension columns | Measure, sex, age, region, adjustment, or other ABS dimensions. | Text | Exact columns come from the ABS DSD. |
| `OBS_VALUE` | Published value. | Text/Numeric | Unit may be people, hours, rate, or percent. |

Use ABS codelists to identify whether a row is a count, rate, original, seasonally adjusted, or trend estimate.
