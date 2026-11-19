# Data.gov.au provider

## Goal

Expose the official Data.gov.au CKAN API through the shared provider registry,
starting with the OAIC aggregate freedom-of-information statistics dataset.

## Requirements

- Use documented public CKAN endpoints.
- Discover the current CSV resource from package metadata.
- Preserve source URL and conditional response metadata.
- Keep the provider separate from Right to Know request archival.
- Provide hermetic CSV parsing and registry tests.

## Serde suitability — 18 November 2026

The Rust root already declares `serde` derive and `serde_json` for provider/model metadata, `csv` for source tables, `reqwest` for HTTP and `polars` for tabular operations. `src/providers/data_gov_au.rs` uses the CSV reader. These are complementary, not competing implementations: replacing CSV parsing, HTTP, dataframes or source-rights checks with Serde would remove required functionality.

- [ ] In the existing provider track, freeze actual CKAN JSON and CSV fixtures, schema/field variants, record counts, row/column alignment, source URLs, temporal IDs, conditional HTTP metadata and licensing/rights attributes.
- [ ] Verify typed Serde provider DTOs only where the official API contract is stable; preserve unknown CKAN fields losslessly where required. Exercise quoting, embedded newlines, UTF-8, duplicate JSON keys, huge inputs, nonfinite/numeric coercion and privacy-safe errors.
- [ ] Compare any manual JSON/CSV transformation with `serde_json`/CSV-deserialize support if and only if a duplicated call site is confirmed. Benchmark memory, correctness and dependency changes; no-go is acceptable.
- [ ] Run exact-head hermetic provider/registry tests plus native security, data-quality and source-rights checks before any replacement. Nothing is implemented by this addendum.
