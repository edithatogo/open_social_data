# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### Added
- **AIHW MyHospitals API Integration (Initial Phase):**
    - Explored MyHospitals API using Swagger definition (`https://myhospitalsapi.aihw.gov.au/swagger/v1/swagger.json`).
    - Created `scripts/shared/aihw_api_fetcher.py` and successfully fetched metadata (measure categories).
    - Selected `/flat-formatted-data-extract/MYH-ED-WAITS` (ED Waiting Times) for initial data implementation.
    - Created repository structure under `datasets/aihw/myhospitals/`.
    - Developed `datasets/aihw/myhospitals/scripts/fetch_aihw_myhospitals_data.py` to fetch paginated data for a measure category and save to Parquet. Successfully tested with a sample from `MYH-ED-WAITS`.
    - Populated initial documentation: `README.md`, `docs/data_dictionary_ed_waits.md`, `docs/accessible_guide_ed_waits.md` for the MyHospitals dataset, focusing on ED Waiting Times.
    - Created `logs/aihw_myhospitals_session_log.md`.
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
