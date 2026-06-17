# Common Data Dictionary Notes for AIHW MyHospitals Flat Extracts

**Source:** AIHW MyHospitals API flat formatted data extracts
**Updated:** 2026-06-18

The MyHospitals flat extract endpoints share a common wide table shape across measure categories. Category-specific data dictionaries remain in this folder, while this note explains fields that recur across the local Parquet files.

| Field Group | Common Columns | Meaning |
|---|---|---|
| Measure identity | `measure_category_code`, `measure_category_name`, `measure_code`, `measure_name` | Identifies the broad measure category and the specific measure. |
| Reported measure | `reported_measure_code`, `reported_measure_name`, `reported_measure_category_*` | Describes the disaggregation reported for a measure, such as triage category, care type, or procedure group. |
| Reporting unit | `reporting_unit_code`, `reporting_unit_name`, `reporting_unit_type_code`, `reporting_unit_type_name` | Identifies the hospital, network, state, or other reporting unit. |
| Geography and grouping | `mapped_state`, `mapped_local_hospital_network`, `mapped_primary_health_network`, `peer_group_code`, `peer_group_name` | Links reporting units to state, network, PHN, and peer-group context. |
| Time period | `data_period`, `data_period_type`, `reporting_start_date`, `reporting_end_date` | Defines the period covered by the observation. |
| Values | `formatted_value`, `raw_value`, `formatted_peer_value`, `raw_peer_value`, `raw_lower_value`, `raw_upper_value` | Contains display values and numeric values where published. |
| Caveats | `caveat`, `caveat_codes`, `caveat_footnotes`, `data_set_caveat*`, `suppression*` | Preserves caveats, footnotes, and legacy suppression fields. |

## Validation Rules Used Locally

The validation script in `scripts/validate_myhospitals_data.py` checks that local Parquet extracts are non-empty and contain core identity, measure, reporting unit, period, and value columns. It also performs a light numeric sanity check on `raw_value` when available.

## Interpretation Notes

`formatted_value` is the safest value for display because it preserves units and caveats. `raw_value` is better for analysis when it is present and numeric, but analysts should keep caveat fields attached to avoid losing suppression or qualification context.
