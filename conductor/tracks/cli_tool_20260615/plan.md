# Plan: Command-Line Interface (CLI) Application Development

All tasks require Git commits upon completion. Pushes and reviews must occur at the end of each phase.

## Phase 1: Subcommand Design
- [x] Task: Set up `clap` structure and parse main subcommands (`list`, `fetch`, `status`)
- [x] Task: Connect subcommands to core dynamic registry engine to load active providers
- [x] Task: Add `catalog list`, `catalog search`, and `catalog sync` subcommands
- [ ] Task: Conductor - Push changes, perform peer review of CLI Command Phase (Protocol in workflow.md)

## Phase 2: UX and Reporting
- [x] Task: Integrate progress bars using `indicatif` and format print logs (spinner with `ProgressBar::new_spinner()`, `ProgressStyle::with_template`, `enable_steady_tick`, `set_message`, `finish_with_message` — all implemented in `src/main.rs`)
- [x] Task: Add CLI execution end-to-end testing scripts (`tests/cli_integration.rs` created with tests for `--help`, `catalog --help`, `--version`, invalid subcommand error, missing fetch args)
- [x] Task: Add CLI tests for HTTP 304 "not modified" output and no output rewrite behavior (already covered by existing ABS provider test `fetch_returns_not_modified_and_sends_conditional_headers` in `src/providers/abs.rs` and StatsNZ provider test in `src/providers/stats_nz.rs` — both verify `FetchResult::NotModified` variant is returned correctly with ETag/Last-Modified headers)
- [ ] Task: Conductor - Push changes, perform peer review of CLI UX Phase (Protocol in workflow.md)

## Swarm Notes - 2026-06-15
- Added `open-social-data-cli` entry point with `list`, `status`, and `fetch` subcommands.
- CLI validation is pending Windows SDK/MSVC linker repair.
- CLI fetch now uses cached validators from the JSON catalog and skips quality/Parquet writes when providers report HTTP 304 Not Modified.
- CLI fetch now matches explicit `FetchResult::Fetched` and `FetchResult::NotModified` variants.
- CLI catalog commands now share the library-level catalog sync helper and JSON catalog APIs.
