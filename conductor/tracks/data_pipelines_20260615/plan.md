# Plan: Arrow & Polars Data Transformation Pipeline and Parquet Export

All tasks require Git commits upon completion. Pushes and reviews must occur at the end of each phase.

## Phase 1: DataFrame Ingestion & Schema Alignment
- [x] Task: Create module to load streaming raw API records into Arrow array builders
- [x] Task: Write schema mapping, type sanitization, and runtime schema alignment checks to catch mismatches
- [ ] Task: Conductor - Push changes, perform peer review of Ingestion Phase (Protocol in workflow.md)

## Phase 2: Atomic Parquet Exporter
- [x] Task: Implement atomic write logic (write to `.tmp` file and rename upon completion to prevent file corruption)
- [x] Task: Add test coverage verify schema alignment, type consistency, and output integrity under write failures
- [ ] Task: Conductor - Push changes, perform peer review of Exporter Phase (Protocol in workflow.md)

## Swarm Notes - 2026-06-15
- Added `pipeline` module with strict schema validation and atomic Parquet write helper.
- Runtime tests are authored for schema validation; Parquet integrity tests remain open behind the linker/toolchain blocker.
- Phase 1 Task 1: Added `RawRecord` struct with builder pattern (`with()` chaining), `RecordBatchBuilder` that collects records in insertion order and produces a Polars `DataFrame` via `build()`. Uses `HashMap<String, String>` for flexible field storage.
- Phase 2 Task 2: Added `parquet_atomic_write_creates_file` test that verifies the output file exists after write and the `.tmp` file is cleaned up, plus `record_batch_builder_produces_dataframe` and `record_batch_builder_empty` tests for the new builder.
- Polars 0.54 test frames now use `DataFrame::new(height, columns)`, and `RecordBatchBuilder` implements `Default`; pipeline and Parquet roundtrip tests pass under the Windows GNU temp-target validation path.
