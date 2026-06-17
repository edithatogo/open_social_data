# Data Dictionary for Consumers Price Index, New Zealand

**Dataset Source:** Stats NZ / Aotearoa Data Explorer
**Updated:** 2026-06-17

| Variable Name | Description | Type | Notes |
|---|---|---|---|
| `TIME_PERIOD` or `Period` | Quarter/month represented by the observation. | Text | Use source frequency metadata. |
| `Expenditure_group` or similar | CPI group, subgroup, class, or item. | Text | Name depends on ADE query. |
| `Series_type` | Index, percentage change, or other selected measure. | Text | Confirm from source metadata. |
| `OBS_VALUE` or `Value` | CPI value or percentage movement. | Numeric/Text | Unit depends on selected measure. |
| `Unit` | Unit of measure if supplied. | Text | Index points or percent are common. |

Recommended checks: non-blank period, non-blank CPI category, numeric values where observations are published.
