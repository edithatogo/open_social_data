# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### Added
- **Rust data engine swarm slice:**
    - Added ABS and Stats NZ provider modules with provider metadata, endpoint URL builders, async HTTP status handling, and row-level dataset parsing.
    - Added a provider registry for loading active providers from the CLI and library code.
    - Added a Polars pipeline module with schema validation and atomic Parquet write support.
    - Added data-quality assertions for non-null checks, numeric ranges, and allowed string values.
    - Extended quality assertions with uniqueness and null-limit checks plus atomic JSON quality report persistence.
    - Added a fetch-side quality gate for provider payload summary frames before Parquet writes.
    - Added a JSON-backed local catalog/cache with atomic persistence for fetched dataset metadata, quality status, quality report paths, and conditional-fetch metadata fields.
    - Added provider source URL propagation into synced catalog entries and conditional request header helpers for cached ETag/Last-Modified values.
    - Added fetch-result catalog merge behavior so fetches do not clear source URLs or provider catalog sync metadata.
    - Added conditional fetch options, provider HTTP 304 handling, and no-rewrite behavior when cached outputs are not modified.
    - Refined provider fetch results into explicit `FetchResult::Fetched` and `FetchResult::NotModified` variants.
    - Added dependency-free mocked HTTP source tests for ABS and Stats NZ conditional request headers and HTTP 304 handling.
    - Added Windows GNU Cargo linker configuration and documented the `C:\tmp` target-dir validation path to avoid Git `link.exe` and OneDrive `target/` ACL issues.
    - Fixed Polars 0.54 test DataFrame construction, added `RecordBatchBuilder` default construction, enabled CLI `--version`, and corrected the CLI integration test binary lookup.
    - Added dependency-light catalog list/search APIs and CLI subcommands over the JSON cache.
    - Added `catalog sync` to sync provider dataset metadata into the JSON-backed local catalog without overwriting existing fetch output or quality fields.
    - Extracted catalog sync into reusable registry/path helpers and `CatalogSyncReport` library support with mock-provider source tests for metadata upserts, missing-provider errors, saved partial provider syncs, and explicit partial-success reporting.
    - Added a SQLite-backed catalog using bundled `rusqlite`, with library load/save/upsert/search helpers and CLI `catalog list/search/sync --sqlite` support.
    - Replaced provider payload-byte summary fetches with ABS SDMX-JSON observation parsing and Stats NZ OData `value` row flattening.
    - Added retry, circuit-breaker, and hardened HTTP client primitives for provider calls.
    - Added a `clap`-based `open-social-data-cli` with `list`, `status`, and `fetch` subcommands plus basic console progress/status formatting.
    - Added a `catalog` CLI subcommand and fetch-side local catalog updates.
    - Updated Conductor track plans to reflect the implemented swarm slice and the remaining local toolchain validation blocker.
- Created `AGENTS.md` with initial guidelines for AI agent contributions.
- **AIHW MyHospitals API Integration:**
    - **MYH-ED-WAITS (ED Waiting Times):**
        - Explored MyHospitals API using Swagger definition.
        - Created `scripts/shared/aihw_api_fetcher.py` for metadata.
        - Selected `/flat-formatted-data-extract/MYH-ED-WAITS`.
        - Created repository structure under `datasets/aihw/myhospitals/`.
        - Developed and tested `datasets/aihw/myhospitals/scripts/fetch_aihw_myhospitals_data.py`, successfully fetching all ~41k records for `MYH-ED-WAITS` and saving to Parquet.
        - Populated documentation: `README.md` (overall MyHospitals), `docs/data_dictionary_ed_waits.md`, `docs/accessible_guide_ed_waits.md`.
        - Created `logs/aihw_myhospitals_session_log.md`.
    - **MYH-ADM (Admissions):**
        - Adapted `fetch_aihw_myhospitals_data.py` for `MYH-ADM`.
        - Tested data fetch for `MYH-ADM` (fetched 5k sample records, full dataset ~112k).
        - Created specific documentation: `docs/data_dictionary_admissions.md`, `docs/accessible_guide_admissions.md`.
        - Updated overall `datasets/aihw/myhospitals/README.md`.
    - **MYH-ES (Elective Surgery):**
        - Adapted `fetch_aihw_myhospitals_data.py` for `MYH-ES`.
        - Tested data fetch for `MYH-ES` (fetched 2k sample records, full dataset ~618k).
        - Created specific documentation: `docs/data_dictionary_elective_surgery.md`, `docs/accessible_guide_elective_surgery.md`.
        - Updated overall `datasets/aihw/myhospitals/README.md`.
    - **MYH-CANCER (Cancer):**
        - Adapted `fetch_aihw_myhospitals_data.py` for `MYH-CANCER`.
        - Tested data fetch for `MYH-CANCER` (successfully fetched all ~2.5k records).
        - Created specific documentation: `docs/data_dictionary_cancer.md`, `docs/accessible_guide_cancer.md`.
        - Updated overall `datasets/aihw/myhospitals/README.md`.
    - **MYH-LOS (Length of Stay):**
        - Adapted `fetch_aihw_myhospitals_data.py` for `MYH-LOS`.
        - Tested data fetch for `MYH-LOS` (fetched 2k sample records, full dataset ~517k).
        - Created specific documentation: `docs/data_dictionary_los.md`, `docs/accessible_guide_los.md`.
        - Updated overall `datasets/aihw/myhospitals/README.md`.
    - **MYH-HH (Hand Hygiene):**
        - Adapted `fetch_aihw_myhospitals_data.py` for `MYH-HH`.
        - Tested data fetch for `MYH-HH` (fetched 2k sample records, full dataset ~38k).
        - Created specific documentation: `docs/data_dictionary_hand_hygiene.md`, `docs/accessible_guide_hand_hygiene.md`.
        - Updated overall `datasets/aihw/myhospitals/README.md`.
- **ABS Dataset - QBIS (Business Indicators, Australia):**
    - Created directory structure under `datasets/abs/qbis_business_indicators/`.
    - Populated initial `README.md`, `docs/data_dictionary.md`, and `docs/accessible_guide.md` from templates.
    - Added `abs_api_fetcher.py` to `scripts/shared/` for common ABS API interactions (DSD fetching).
    - Updated `abs_dataset_session_log.md` with progress.
    - Updated `TODO.md` with specific tasks for QBIS data fetching and processing.
    - *Note: DSD and data fetching for ABS QBIS are ON HOLD due to persistent network resolution issues for `api.abs.gov.au` in the environment. Awaiting instructions for AIHW dataset.*

### Changed
- Moved `abs_api_fetcher.py` (formerly `list_abs_datasets.py`) to `scripts/shared/` and updated its DSD fetching capabilities.
- Marked ABS QBIS data fetching tasks in `TODO.md` as ON HOLD.

## [Previous Date - from template, replace with actual date of these changes if known] - Initial Setup
### Added
- Initial project philosophy, goals, and repository structure established.
- Core documentation templates (`dataset_readme_template.md`, `data_dictionary_template.md`, `accessible_guide_template.md`) created.
- Initial content for `README.md` (main), `CHANGELOG.md`, `ROADMAP.md`, `TODO.md` and `SESSION_LOG_TEMPLATE.md` written.
- Placeholders for `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md` created.

## [YYYY-MM-DD] - Project Initiation
### Added
- Repository created.
- Basic placeholder files for `README.md`, `LICENSE`.
