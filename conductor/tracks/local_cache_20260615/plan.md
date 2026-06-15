# Plan: Local Metadata Caching and SQLite/DuckDB Cataloging

All tasks require Git commits upon completion. Pushes and reviews must occur at the end of each phase.

## Phase 1: SQLite/DuckDB Schema Integration
- [ ] Task: Set up local database connection pool and initialize schema for agency catalogs
- [ ] Task: Write syncer module to download and cache complete API catalogs into database
- [ ] Task: Conductor - Push changes, perform peer review of Database Setup Phase (Protocol in workflow.md)

### Dependency-light cache step
- [x] Task: Define JSON cache metadata schema for providers, datasets, fetch timestamps, and output artifacts
- [x] Task: Implement atomic JSON catalog persistence for fetched dataset metadata
- [x] Task: Wire CLI `catalog` readout and fetch-side catalog updates
- [x] Task: Add dependency-light catalog list and search APIs over JSON cache
- [x] Task: Wire CLI catalog sync to provider metadata listing and JSON catalog upserts

## Phase 2: Metadata Search Interface
- [x] Task: Implement CLI search commands and programmatic metadata queries
- [x] Task: Write source-level tests validating catalog syncer logic
- [ ] Task: Write tests validating local search reliability after Windows SDK/MSVC linker repair
- [ ] Task: Run catalog syncer and local search tests after Windows SDK/MSVC linker repair
- [ ] Task: Conductor - Push changes, perform peer review of Search Phase (Protocol in workflow.md)

## Swarm Notes - 2026-06-15
- Added JSON-backed local catalog as a no-native-dependency first pass because the current machine cannot link MSVC/Windows SDK binaries.
- SQLite/DuckDB storage remains open until the native linker/toolchain blocker is resolved.
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
