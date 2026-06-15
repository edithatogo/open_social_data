# Execution Strategy

## Objective
The session was initiated with a general prompt ("chat chat"). Based on the repository state, there are multiple active tracks defined in `conductor/tracks.md`. The most foundational track is **Track 1: Scaffold Rust data engine core and define DatasetProvider trait** (`conductor/tracks/rust_core_20260615/`). 

Since no specific track was requested, the strategy is to proceed with implementing **Track 1**.

## Key Files & Context
- `conductor/tracks.md`
- `conductor/tracks/rust_core_20260615/spec.md`
- `conductor/tracks/rust_core_20260615/plan.md`

## Implementation Steps
1. Execute Phase 1 of Track 1:
   - Initialize the `cargo` library project in the root workspace.
   - Configure `Cargo.toml` with the specified dependencies (`tokio`, `reqwest`, `polars`, `serde`, `thiserror`).
2. Execute Phase 2 of Track 1:
   - Define standard data structures for dataset metadata.
   - Define the `DatasetProvider` trait in Rust.
3. Execute Phase 3 of Track 1:
   - Implement a `MockProvider` conforming to the trait.
   - Write unit and integration tests.

## Verification & Testing
- Run `cargo check` and `cargo fmt`.
- Run `cargo test` to ensure the mock provider and traits are correctly implemented.
