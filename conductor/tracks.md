# Project Tracks

This file tracks all major tracks for the project. Each track has its own detailed plan in its respective folder.

---

## [x] Track 1: Scaffold Rust data engine core and define DatasetProvider trait
*Link: [./conductor/tracks/rust_core_20260615/](./conductor/tracks/rust_core_20260615/)*

## [x] Track 2: Implement agency API providers (ABS, Stats NZ)
*Link: [./conductor/tracks/api_integrations_20260615/](./conductor/tracks/api_integrations_20260615/)*
- ABS and Stats NZ providers implemented with SDMX/OData support
- HTTP 304 conditional request handling with ETag/Last-Modified
- Unit tests with mock HTTP server for both providers

## [x] Track 3: Arrow & Polars data transformation pipeline and Parquet export
*Link: [./conductor/tracks/data_pipelines_20260615/](./conductor/tracks/data_pipelines_20260615/)*
- Schema validation with `ExpectedColumn` and `validate_schema`
- Atomic Parquet writer (`write_parquet_atomic`)
- `RawRecord` + `RecordBatchBuilder` for flexible DataFrame construction
- `read_parquet` helper

## [x] Track 4: Command-Line Interface (CLI) application development
*Link: [./conductor/tracks/cli_tool_20260615/](./conductor/tracks/cli_tool_20260615/)*
- `list`, `status`, `fetch`, `catalog list/search/sync` subcommands via clap
- Progress bars with indicatif
- Integration tests at `tests/cli_integration.rs`

## [x] Track 5: Ingestion optimization and pipeline hardening
*Link: [./conductor/tracks/hardening_opt_20260615/](./conductor/tracks/hardening_opt_20260615/)*
- Retry policy with exponential backoff
- Circuit breaker pattern
- `run_provider_safely` for panic isolation
- Pre-configured HTTP client with timeouts

## [x] Track 6: Data quality assertions and delta updates
*Link: [./conductor/tracks/data_quality_20260615/](./conductor/tracks/data_quality_20260615/)*
- `QualityAssertion` enum (NonNull, NullLimit, Unique, NumericRange, AllowedValues)
- `QualityReport` with atomic JSON persistence
- `DeltaUpdater` for incremental Parquet appends
- Quality gates in CLI fetch flow

## [x] Track 7: Documentation and tool registry publication
*Link: [./conductor/tracks/doc_registry_20260615/](./conductor/tracks/doc_registry_20260615/)*
- Module-level rustdoc on all source files
- User guides for reading Parquet in Python and R
- Cargo.toml metadata (license, keywords, repository, description)
- GitHub Actions CI workflow (check, fmt, clippy, test, release)

## [~] Track 8: Local metadata caching and SQLite/DuckDB cataloging
*Link: [./conductor/tracks/local_cache_20260615/](./conductor/tracks/local_cache_20260615/)*
- [x] JSON-backed local catalog with atomic saves, search, and sync
- [ ] SQLite/DuckDB integration — blocked on Windows SDK/MSVC linker repair
