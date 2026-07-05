# Track Plan: DigitalNZ Provider and DNZ CLI Capability Alignment

## GitHub Issues

- Provider core: https://github.com/edithatogo/open_social_data/issues/5
- New Zealand Gazette dataset: https://github.com/edithatogo/open_social_data/issues/4
- CLI and documentation alignment: https://github.com/edithatogo/open_social_data/issues/3
- Upstream DNZ compatibility contract: https://github.com/edithatogo/dnz/issues/3
- Upstream DNZ live registry submission: https://github.com/edithatogo/dnz/issues/4

## Upstream DNZ Evidence

- DNZ commit: https://github.com/edithatogo/dnz/commit/d09a767
- DNZ release: https://github.com/edithatogo/dnz/releases/tag/v0.1.0
- DNZ MCPB bundle: https://github.com/edithatogo/dnz/releases/download/v0.1.0/dnz-mcp-0.1.0.mcpb
- DNZ MCPB SHA-256: `c06f3c4da99b24d3d70545df2e4c802f9d4ecbdb7f4323991d78d104deb41ee6`
- DNZ MCP Registry metadata validates with `mcp-publisher validate registry/mcp/server.draft.json`.
- DNZ live MCP Registry publish remains blocked by expired/invalid MCP Registry JWT.
- DNZ Smithery submission was attempted under namespace `edithatogo`; Smithery rejected the valid MCPB with `400 Invalid input: expected object, received undefined`.
- DNZ crates.io and PyPI live publishing remain blocked by missing registry tokens in the current environment.

## Phase 1: Boundary and Contract

- [x] Task: Confirm the integration boundary between `dnz` and `open_social_data`.
    - [x] Document `dnz` responsibilities: ad hoc search, facets, Gazette export, cache, MCP, Python/Rust package surfaces.
    - [x] Document `open_social_data` responsibilities: curated provider fetches, catalog entries, quality reports, and Parquet outputs.
    - [x] Decide whether to depend on `dnz-core` directly or ingest `dnz` JSONL/manifest outputs first.
- [x] Task: Define the DigitalNZ provider contract.
    - [x] Add curated dataset ID definitions for `nz_gazette` and a small fixture-backed search dataset.
    - [x] Define authentication and cache configuration behavior.
    - [x] Define output columns and provenance fields.

## Phase 2: Provider Implementation

- [x] Task: Add a `digitalnz` provider module.
    - [x] Implement `DatasetProvider::metadata`.
    - [x] Implement `ping` without leaking credentials.
    - [x] Implement `list_datasets` for curated datasets.
    - [x] Implement `fetch_dataset_with_options` for fixture-backed and live-capable fetches.
- [x] Task: Register the provider.
    - [x] Add `DigitalNzProvider` to `ProviderRegistry::with_defaults`.
    - [x] Add unit tests for registration and provider lookup.

## Phase 3: Gazette Dataset Workflow

- [x] Task: Normalize New Zealand Gazette records into the open social data shape.
    - [x] Map DNZ record fields into stable DataFrame columns.
    - [x] Preserve source URLs, content partner, collection/category, dates, and provider-specific extra metadata.
    - [x] Add quality assertions for required Gazette fields.
- [x] Task: Add catalog and fixture coverage.
    - [x] Add hermetic fixture data for Gazette/search responses.
    - [x] Add tests for catalog sync/list/search behavior with `digitalnz`.

## Phase 4: CLI and Documentation Alignment

- [x] Task: Add user-facing docs for command mapping.
    - [x] Explain `dnz search` versus `open-social-data-cli fetch digitalnz ...`.
    - [x] Explain `dnz gazette-export` versus curated `digitalnz nz_gazette` fetches.
    - [x] Document required environment variables and cache behavior.
- [x] Task: Add CLI reference/examples.
    - [x] Add example fetch commands.
    - [x] Add catalog sync/list/search examples for DigitalNZ datasets.
    - [x] Add troubleshooting notes for missing `DIGITALNZ_API_KEY`.

## Phase 5: Validation and Release Readiness

- [x] Task: Run local validation.
    - [x] Run `cargo fmt --check`.
    - [x] Run `cargo check`.
    - [x] Run `cargo clippy -- -D warnings`.
    - [x] Run unit and CLI integration tests.
- [x] Task: Verify GitHub issue and Conductor alignment.
    - [x] Ensure GitHub issues link back to this track.
    - [x] Update this plan with implementation evidence and commit SHAs.
    - [x] Prepare follow-up work for wider DNZ dataset coverage after `nz_gazette`.

## Implementation Evidence

- Commit: `01920d5` (`feat: add DigitalNZ curated provider`)
- Validation: `cargo +stable-x86_64-pc-windows-gnu check --all-targets`
- Validation: `cargo +stable-x86_64-pc-windows-gnu test --all-targets`
- Validation: `cargo +stable-x86_64-pc-windows-gnu clippy --all-targets -- -D warnings`
