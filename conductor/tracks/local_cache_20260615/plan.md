# Plan: Local Metadata Caching and SQLite/DuckDB Cataloging

All tasks require Git commits upon completion. Pushes and reviews must occur at the end of each phase.

## Phase 1: SQLite/DuckDB Schema Integration
- [x] Task: Set up local database connection and initialize schema for agency catalogs
- [x] Task: Write syncer module to download and cache complete API catalogs into database
- [x] Task: Conductor - Review database setup phase against JSON and SQLite catalog tests

### Dependency-light cache step
- [x] Task: Define JSON cache metadata schema for providers, datasets, fetch timestamps, and output artifacts
- [x] Task: Implement atomic JSON catalog persistence for fetched dataset metadata
- [x] Task: Wire CLI `catalog` readout and fetch-side catalog updates
- [x] Task: Add dependency-light catalog list and search APIs over JSON cache
- [x] Task: Wire CLI catalog sync to provider metadata listing and JSON catalog upserts

## Phase 2: Metadata Search Interface
- [x] Task: Implement CLI search commands and programmatic metadata queries
- [x] Task: Write source-level tests validating catalog syncer logic
- [x] Task: Write tests validating local search reliability under the Windows GNU toolchain
- [x] Task: Run catalog syncer and local search tests using `CARGO_TARGET_DIR=C:\tmp\open_social_data_target2`
- [x] Task: Conductor - Review search phase against local catalog search tests

## Swarm Notes - 2026-06-15
- Added JSON-backed local catalog as a no-native-dependency first pass because the current machine cannot link MSVC/Windows SDK binaries.
- SQLite-backed storage is implemented and validated; DuckDB remains out of scope because SQLite satisfies the completed catalog requirement.
- Replaced the initial flat `catalog` command with `catalog list` and `catalog search`.
- Extended JSON catalog metadata with ETag, Last-Modified, source URL, quality status, quality report path, and not-modified counters.
- Added `catalog sync` to pull provider dataset metadata into the JSON catalog while preserving existing fetch output and quality fields.
- Provider catalog sync now propagates dataset source URLs into the JSON catalog where available.
- Fetch-side catalog updates now merge output and quality fields without clearing source URLs or catalog sync metadata.
- Fetch-side catalog updates now reuse cached ETag/Last-Modified metadata and record not-modified events without rewriting outputs.
- Not-modified catalog updates are now driven by explicit `FetchResult::NotModified` responses.
- Extracted catalog sync into a reusable library helper with a sync report and mock-provider source tests.
- Catalog sync reports now explicitly flag partial provider success when some providers sync before later provider errors; the CLI reports saved partial results after the catalog write succeeds.
- Added a path-level catalog sync helper that load-sync-saves atomically and source-tests partial provider sync records on disk.
- Windows GNU validation now uses repo-local `gcc` linker config plus `CARGO_TARGET_DIR=C:\tmp\open_social_data_target2` to avoid Git `link.exe` and OneDrive target ACL issues.

## Implementation Notes - 2026-06-17
- Added a `rusqlite`/bundled-SQLite catalog backend that persists the existing `CachedDataset` schema in an embedded database.
- Added SQLite catalog sync, list, and search support through `catalog --sqlite` CLI options while keeping JSON as the default catalog path.
- The earlier native-linker blocker is stale for this track; validation now runs under the Windows GNU toolchain using `CARGO_TARGET_DIR=C:\tmp\open_social_data_target2`.
