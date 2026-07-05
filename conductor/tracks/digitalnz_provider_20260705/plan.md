# Track Plan: DigitalNZ Provider and DNZ CLI Capability Alignment

## GitHub Issues

- Provider core: https://github.com/edithatogo/open_social_data/issues/5
- New Zealand Gazette dataset: https://github.com/edithatogo/open_social_data/issues/4
- CLI and documentation alignment: https://github.com/edithatogo/open_social_data/issues/3
- Upstream DNZ compatibility contract: https://github.com/edithatogo/dnz/issues/3

## Phase 1: Boundary and Contract

- [ ] Task: Confirm the integration boundary between `dnz` and `open_social_data`.
    - [ ] Document `dnz` responsibilities: ad hoc search, facets, Gazette export, cache, MCP, Python/Rust package surfaces.
    - [ ] Document `open_social_data` responsibilities: curated provider fetches, catalog entries, quality reports, and Parquet outputs.
    - [ ] Decide whether to depend on `dnz-core` directly or ingest `dnz` JSONL/manifest outputs first.
- [ ] Task: Define the DigitalNZ provider contract.
    - [ ] Add curated dataset ID definitions for `nz_gazette` and a small fixture-backed search dataset.
    - [ ] Define authentication and cache configuration behavior.
    - [ ] Define output columns and provenance fields.

## Phase 2: Provider Implementation

- [ ] Task: Add a `digitalnz` provider module.
    - [ ] Implement `DatasetProvider::metadata`.
    - [ ] Implement `ping` without leaking credentials.
    - [ ] Implement `list_datasets` for curated datasets.
    - [ ] Implement `fetch_dataset_with_options` for fixture-backed and live-capable fetches.
- [ ] Task: Register the provider.
    - [ ] Add `DigitalNzProvider` to `ProviderRegistry::with_defaults`.
    - [ ] Add unit tests for registration and provider lookup.

## Phase 3: Gazette Dataset Workflow

- [ ] Task: Normalize New Zealand Gazette records into the open social data shape.
    - [ ] Map DNZ record fields into stable DataFrame columns.
    - [ ] Preserve source URLs, content partner, collection/category, dates, and provider-specific extra metadata.
    - [ ] Add quality assertions for required Gazette fields.
- [ ] Task: Add catalog and fixture coverage.
    - [ ] Add hermetic fixture data for Gazette/search responses.
    - [ ] Add tests for catalog sync/list/search behavior with `digitalnz`.

## Phase 4: CLI and Documentation Alignment

- [ ] Task: Add user-facing docs for command mapping.
    - [ ] Explain `dnz search` versus `open-social-data-cli fetch digitalnz ...`.
    - [ ] Explain `dnz gazette-export` versus curated `digitalnz nz_gazette` fetches.
    - [ ] Document required environment variables and cache behavior.
- [ ] Task: Add CLI reference/examples.
    - [ ] Add example fetch commands.
    - [ ] Add catalog sync/list/search examples for DigitalNZ datasets.
    - [ ] Add troubleshooting notes for missing `DIGITALNZ_API_KEY`.

## Phase 5: Validation and Release Readiness

- [ ] Task: Run local validation.
    - [ ] Run `cargo fmt --check`.
    - [ ] Run `cargo check`.
    - [ ] Run `cargo clippy -- -D warnings`.
    - [ ] Run unit and CLI integration tests.
- [ ] Task: Verify GitHub issue and Conductor alignment.
    - [ ] Ensure GitHub issues link back to this track.
    - [ ] Update this plan with implementation evidence and commit SHAs.
    - [ ] Prepare follow-up work for wider DNZ dataset coverage after `nz_gazette`.
