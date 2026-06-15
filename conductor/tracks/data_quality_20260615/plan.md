# Plan: Data Quality Assertions and Delta Updates

All tasks require Git commits upon completion. Pushes and reviews must occur at the end of each phase.

## Phase 1: Data Quality Check Framework
- [x] Task: Design quality assertion models (value boundary checks, non-null guarantees)
- [x] Task: Hook validation engine into Polars preprocessing execution pipeline
- [ ] Task: Conductor - Push changes, perform peer review of Data Quality Phase (Protocol in workflow.md)

## Phase 2: Delta Updates
- [x] Task: Set up metadata schema tracking last fetched data ranges and HTTP timestamp checks
- [ ] Task: Implement dynamic range requests and incremental appending into target Parquet files
- [ ] Task: Add test suites verifying incremental loads and invalid schema assertions
- [ ] Task: Conductor - Push changes, perform peer review of Delta Phase (Protocol in workflow.md)

## Swarm Notes - 2026-06-15
- Added `quality` module with non-null, numeric range, and allowed-value assertions over Polars data frames.
- Added unit tests for passing assertions and range failures; full test execution remains blocked by the local Windows SDK/MSVC linker setup.
- Extended quality assertions with uniqueness and null-limit checks.
- Added atomic JSON quality report persistence and optional CLI `fetch --quality-report` output.
- Added a fetch-side quality gate before Parquet writes.
- Added quality status persistence into cached dataset metadata.
- Added cached ETag/Last-Modified fields and conditional request header helpers.
- Added provider conditional fetch options and HTTP 304 Not Modified handling through explicit `FetchResult::NotModified` so CLI fetches preserve existing output files.
