# Plan: Documentation and Tool Registry Publication

All tasks require Git commits upon completion. Pushes and reviews must occur at the end of each phase.

## Phase 1: Code Documentation & Accessible Guides
- [x] Task: Complete Rust standard `cargo doc` annotations and verify output builds locally
- [x] Task: Author user-facing accessible markdown guides on reading Parquet in external tools (Python, with R later retired)
- [ ] Task: Conductor - Push changes, perform peer review of Documentation Phase (Protocol in workflow.md)

## Phase 2: Registry Publishing Configuration
- [x] Task: Configure crate names, metadata, keywords, and license in `Cargo.toml`
- [ ] Task: Execute package publication dry run `cargo publish --dry-run` and configure cargo credentials setup
- [x] Task: Add GitHub Actions CI/CD workflow file to build release assets and run test checks
- [ ] Task: Conductor - Push changes, perform peer review of Publishing Phase (Protocol in workflow.md)

## Swarm Implementation Notes

### Phase 1 Task 1 - Rustdoc Annotations
- Added `//!` module-level doc comments to all 13 source files:
  - `src/lib.rs` - crate-level architecture overview
  - `src/traits.rs` - DatasetProvider, FetchOptions, FetchResult docs
  - `src/models.rs` - ProviderMetadata, DatasetMetadata, Catalog docs
  - `src/error.rs` - CoreError variants documentation
  - `src/registry.rs` - ProviderRegistry docs
  - `src/catalog.rs` - LocalCatalog, CachedDataset docs
  - `src/pipeline.rs` - RawRecord, RecordBatchBuilder, validate_schema, write_parquet_atomic docs
  - `src/hardening.rs` - RetryPolicy, CircuitBreaker, build_http_client, run_provider_safely docs
  - `src/quality.rs` - QualityAssertion, validate_quality, DeltaUpdater docs
  - `src/mock.rs` - MockProvider docs
  - `src/providers/mod.rs` - provider module docs
  - `src/providers/abs.rs` - AbsProvider docs
  - `src/providers/stats_nz.rs` - StatsNzProvider docs
- Added `///` item-level doc comments on all public structs, enums, methods, and trait definitions in `traits.rs` and `models.rs`

### Phase 1 Task 2 - User Guides
- Created `docs/guides/reading-parquet-in-python.md` - guide for pandas and polars users
- Retired `docs/guides/reading-parquet-in-r.md` after consolidating examples under the Rust CLI
- Created `docs/guides/README.md` - index linking to both guides

### Phase 2 Task 1 - Cargo.toml Metadata
- Added `description`, `keywords`, `categories`, `license`, `repository`, `homepage`, and `readme` fields to `[package]` section

### Phase 2 Task 3 - GitHub Actions CI
- Created `.github/workflows/ci.yml` with jobs for:
  - `cargo check` - verify compilation
  - `cargo fmt --check` - formatting lint
  - `cargo clippy --all-targets -- -D warnings` - clippy lint
  - `cargo test` - run all tests
  - `build-release` - tagged release builds with artifact upload
