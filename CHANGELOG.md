# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]
### Added
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
