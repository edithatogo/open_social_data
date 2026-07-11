# Test Fixtures

This directory contains synthetic/minimised fixture payloads used by unit tests
in the `open_social_data` crate. **None of the data values are official
statistics.** They are illustrative examples matching the wire formats returned
by the upstream APIs.

## Fixture sourcing

| File | Source API | Purpose |
|---|---|---|
| `abs_sdmx_response.json` | [ABS SDMX-JSON API](https://api.abs.gov.au/) | ABS SDMX-JSON data response (v1.0.0) |
| `digitalnz_nz_gazette_response.json` | [DigitalNZ records API](https://api.digitalnz.org/v3/records.json) | Curated New Zealand Gazette search response for provider tests |
| `digitalnz_search_fixture_response.json` | [DigitalNZ records API](https://api.digitalnz.org/v3/records.json) | General DigitalNZ search fixture response for provider validation |
| `stats_nz_odata_response.json` | [Stats NZ OData v1 API](https://api.stats.govt.nz/opendata/v1) | Stats NZ OData JSON entity set response |

## ABS SDMX-JSON (`abs_sdmx_response.json`)

- Format: [SDMX-JSON 1.0.0](https://github.com/sdmx-twg/sdmx-json), the wire
  format returned by `GET /data/{id}?detail=full&dimensionAtObservation=AllDimensions`.
- Structure mirrors the test payload in `src/providers/abs.rs`:
  - 2 series dimensions: `MEASURE` (SALES, PROFIT) and `REGION` (AUS).
  - 1 observation dimension: `TIME_PERIOD` (2024-Q1, 2024-Q2).
  - 4 observations (2 measures × 1 region × 2 time periods).
- Values are arbitrary and do not represent any real ABS release.

## Stats NZ OData (`stats_nz_odata_response.json`)

- Format: OData v1 JSON entity set, the wire format returned by
  `GET /opendata/v1/{entity-set}`.
- Structure follows the test payload in `src/providers/stats_nz.rs`:
  - `value` array with 3 records.
  - Fields: `Measure`, `Region`, `TimePeriod`, `Value`, `Unit`.
- Values are arbitrary and do not represent any real Stats NZ release.

## DigitalNZ Gazette (`digitalnz_nz_gazette_response.json`)

- Format: DigitalNZ search response JSON returned by `GET /v3/records.json`.
- Structure mirrors the `nz_gazette` provider dataset contract:
  - `result_count` with two Gazette records.
  - Fields include `collection`, `content_partner`, `category`, `date`, `display_url`, `source_url`, and `syndication_date`.
  - Extra provider fields such as `license` and `usage` are preserved in the provider's `extra_fields_json` column.

## DigitalNZ Search Fixture (`digitalnz_search_fixture_response.json`)

- Format: DigitalNZ search response JSON returned by `GET /v3/records.json`.
- Structure mirrors the provider validation dataset contract:
  - Two curated search records.
  - Fields include `collection`, `content_partner`, `category`, `creator`, `date`, `display_url`, `source_url`, and `syndication_date`.
  - Extra provider fields are preserved in the provider's `extra_fields_json` column.

## Creation and minimisation

These files were created by hand from the embedded `serde_json::json!` macro
calls in the existing unit tests of the providers, then exported as standalone
JSON files. No automated minimisation or redaction was performed because the
payloads are already compact.

## Licence

The fixture files in this directory are part of the open_social_data project
and are licensed under the same terms as the project itself. They do not
incorporate any official ABS or Stats NZ data.
