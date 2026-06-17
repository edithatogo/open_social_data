# Data Dictionary for Labour Market Statistics, New Zealand

**Dataset Source:** Stats NZ / Aotearoa Data Explorer
**Updated:** 2026-06-17

| Variable Name | Description | Type | Notes |
|---|---|---|---|
| `TIME_PERIOD` or `Period` | Observation period. | Text | Frequency depends on selected table. |
| `Measure` | Labour-market measure such as employed, unemployed, unemployment rate, or participation rate. | Text | Confirm labels from ADE metadata. |
| `Age` | Age group, if selected. | Text | Optional dimension. |
| `Sex` | Sex category, if selected. | Text | Optional dimension. |
| `Region` | Geographic area, if selected. | Text | Optional dimension. |
| `OBS_VALUE` or `Value` | Published value. | Numeric/Text | Unit can be people, rate, or percent. |

Recommended checks: non-blank period and measure; values numeric after excluding suppressed observations.
