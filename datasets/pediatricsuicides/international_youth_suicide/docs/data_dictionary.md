# Proposed canonical observation contract

**Status: our target contract, not the uninspected upstream CSV schema.** Do not
write a source parser by guessing headers from this table. Preserve original
columns and map only after capturing a source export and its dictionary.

| Field group | Required meaning and checks |
|---|---|
| `source_id`, `source_revision`, `source_row_id` | Stable source identity, exact release/byte identity and reversible row locator. |
| `jurisdiction_id`, `jurisdiction_version` | Source-native identifier plus a reviewed geographic concordance; historical boundaries are explicit. |
| `period_start`, `period_end`, `time_basis` | Observation interval and whether deaths are by occurrence, registration or another defined basis. Preserve release date separately in lineage. |
| `age_min_inclusive`, `age_max_exclusive` | Exact non-overlapping age boundaries, including under-one conventions. Never silently interpolate age bands. |
| `sex_source_label` | Original supplied label; not-supplied is distinct from an explicitly supplied all-sex aggregate. |
| `cause_definition` | Source ICD revision/code set or documented classification; do not assume identical cause coverage across sources. |
| `deaths` | Non-negative integer when observed. Null remains null for missing/suppressed/unavailable values. |
| `exposure`, `exposure_unit` | Positive denominator and its original definition, including reference/mid-year population versus person-years. |
| `value_status`, `coverage_status` | Explicit observed, missing, suppressed or unavailable state and reporting-area/completeness information. |
| `upstream_packet_sha256` | SHA-256 of the immutable source payload; never fabricate it from a URL or rendered page. |

Retain source-provided pooling/exclusion flags and identify the underlying death
register in lineage. The same deaths accessed via WHO, ABS or a curator are not
independent observations. A candidate key includes source revision, jurisdiction,
period, age band, sex label, cause definition and coverage, not jurisdiction-year
alone. Row count is recorded only after parsing the exact export.

Compute rates only where numerator, denominator, age, geography and period agree.
For a pooled rate, sum compatible counts and corresponding exposures before
dividing; do not average rates. Do not sum all-sex and sex-specific rows, national
and constituent regional rows, overlapping historic jurisdictions, or overlapping
age bands. Do not reconstruct suppressed cells by subtraction. Modelled estimates
must be separately labelled, with assumptions and uncertainty, never blended into
observed data.

Price and cost panels additionally require currency, price year, constant/current
price basis, conversion series/vintage and economic perspective. Policy panels
require announcement and effective dates, eligibility, source clauses and an
independent applicability review. Changes in reporting practices are not policy
effects by default.
