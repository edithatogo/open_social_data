# Project TODO List

This file tracks specific, actionable tasks for the Open Social Datasets project. Items will be moved from the `ROADMAP.md` to here as they become current.

## Phase 1: Foundation & Initial Datasets

### Core Setup (Most items completed, see CHANGELOG)
*   [X] Define Project Philosophy and Goals.
*   [X] Establish Repository Structure.
*   [X] Develop Core Documentation Templates (`dataset_readme_template.md`, `data_dictionary_template.md`, `accessible_guide_template.md`).
*   [X] Write Initial Content for `README.md`, `CHANGELOG.md`, `ROADMAP.md`, `TODO.md`, `SESSION_LOG_TEMPLATE.md`.
*   [X] **Create `AGENTS.md`:** Add initial instructions for AI agent contributions to the root directory. (Done)
*   [X] Add Rust data engine scaffold with provider trait, ABS/Stats NZ providers, CLI, quality checks, Parquet export, and JSON-backed local catalog.
*   [X] Add dependency-light `catalog sync` CLI/local metadata sync using provider dataset listings, JSON catalog upserts, and a reusable library helper.
*   [X] Extract catalog sync into a library helper with mock-provider coverage.
*   [X] Resolve local Windows Rust/GNU linker setup so `cargo check`, `cargo clippy`, and tests can run end-to-end with `CARGO_TARGET_DIR=C:\tmp\open_social_data_target2`.
*   [X] Add SQLite-backed catalog as a follow-up to the JSON-backed cache.
*   [X] Replace provider payload-summary fetches with row-level ABS SDMX and Stats NZ OData parsing.
*   [X] Add cached ETag/Last-Modified metadata fields and conditional request header helpers.
*   [X] Wire cached ETag/Last-Modified metadata into provider fetches.
*   [X] Handle HTTP 304 Not Modified in CLI fetch without rewriting outputs.
*   [X] Refactor provider fetch results into explicit fetched/not-modified variants.
*   [X] Add source-level mocked tests for provider HTTP 304 handling; run them once local Cargo linking is working.
*   [X] Add source-level mocked tests for `catalog sync` provider metadata upserts.
*   [X] Run mocked `catalog sync` tests once local Cargo linking is working.

### Initial Dataset Population
*   **Stats NZ Datasets (ON HOLD):**
    *   [ ] Identify and prioritize 2-3 key datasets from Statistics New Zealand (Stats NZ).
    *   [ ] For Dataset 1 (Stats NZ):
        *   [ ] Create directory: `datasets/stats_nz/[dataset_name_1]/`
        *   [ ] Populate `README.md` from template.
        *   [ ] Develop `data_dictionary.md`.
        *   [ ] Write `accessible_guide.md`.
        *   [ ] Add data access details/scripts.
        *   [ ] Log session in `SESSION_LOG.md` (using template).
    *   [ ] For Dataset 2 (Stats NZ):
        *   [ ] Create directory: `datasets/stats_nz/[dataset_name_2]/`
        *   [ ] Populate `README.md` from template.
        *   [ ] Develop `data_dictionary.md`.
        *   [ ] Write `accessible_guide.md`.
        *   [ ] Add data access details/scripts.
        *   [ ] Log session.
*   **ABS Datasets:**
    *   [ ] Identify and prioritize 2-3 key datasets from the Australian Bureau of Statistics (ABS).
    *   [X] For Dataset 1 (ABS - QBIS Business Indicators):
        *   [X] Create directory: `datasets/abs/qbis_business_indicators/` (Done)
        *   [X] Populate `README.md` from template (Initial version done)
        *   [X] Develop `data_dictionary.md` (Initial version done, pending DSD confirmation)
        *   [X] Write `accessible_guide.md` (Initial version done)
        *   [ ] **Develop `fetch_qbis_data.py` script in `datasets/abs/qbis_business_indicators/scripts/` (ON HOLD - Network Issues):**
            *   [ ] Implement function to fetch data from `https://api.abs.gov.au/data/QBIS`.
            *   [ ] Handle potential API query parameters (filters for dimensions like industry, region, specific indicators, date range).
            *   [ ] Parse SDMX-JSON data into a Pandas DataFrame.
            *   [ ] Save DataFrame to Parquet format in `datasets/abs/qbis_business_indicators/data/`.
            *   [ ] Add error handling and logging.
        *   [ ] **Verify/Update QBIS `data_dictionary.md`** once live DSD/data is accessible (ON HOLD).
        *   [X] **Resolve Network Access Issues** for `api.abs.gov.au` if possible, or note workarounds (Issue noted, resolution out of scope for agent).
        *   [X] Log session (Ongoing in `abs_dataset_session_log.md`).
    *   [ ] For Dataset 2 (ABS):
        *   [ ] Create directory: `datasets/abs/[dataset_name_2]/`
        *   [ ] Populate `README.md` from template.
        *   [ ] Develop `data_dictionary.md`.
        *   [ ] Write `accessible_guide.md`.
        *   [ ] Add data access details/scripts.
        *   [ ] Log session.
*   **AIHW Datasets:**
    *   [X] Identify and prioritize dataset(s) from the Australian Institute of Health and Welfare (AIHW) (User selected MyHospitals API).
    *   [X] For AIHW MyHospitals - `MYH-ED-WAITS` (ED Waiting Times) measure category:
        *   [X] Create directory structure.
        *   [X] Explore API documentation.
        *   [X] Perform initial API interaction.
        *   [X] Select specific endpoint.
        *   [X] Develop `fetch_aihw_myhospitals_data.py` script.
        *   [X] Populate initial documentation.
        *   [X] Test full run of script (Completed, ~41k records fetched and saved).
        *   [ ] **Refine `data_dictionary_ed_waits.md`** based on full dataset analysis if needed.
        *   [ ] Consider adding scripts/notebooks for data validation or example queries.
        *   [X] Log session.
    *   [X] For AIHW MyHospitals - `MYH-ADM` (Admissions) measure category:
        *   [X] Identify next measure category.
        *   [X] Adapt/extend script.
        *   [X] Test data fetch (Completed with 5-page limit, ~5k records fetched, full dataset ~112k).
        *   [X] Create specific documentation.
        *   [X] Update overall MyHospitals README.
        *   [ ] **Refine `data_dictionary_admissions.md`** based on full dataset analysis if feasible.
        *   [ ] Consider adding scripts/notebooks for data validation or example queries.
    *   [X] For AIHW MyHospitals - `MYH-ES` (Elective Surgery) measure category:
        *   [X] Identify next measure category.
        *   [X] Adapt/extend script.
        *   [X] Test data fetch (Completed with 2-page limit, ~2k records fetched, full dataset ~618k).
        *   [X] Create specific documentation.
        *   [X] Update overall MyHospitals README.
        *   [ ] **Refine `data_dictionary_elective_surgery.md`** based on full dataset analysis if feasible.
        *   [ ] Consider adding scripts/notebooks for data validation or example queries.
    *   [X] For AIHW MyHospitals - `MYH-CANCER` (Cancer) measure category:
        *   [X] Identify next measure category.
        *   [X] Adapt/extend script.
        *   [X] Test data fetch (Completed, all ~2.5k records fetched and saved).
        *   [X] Create specific documentation.
        *   [X] Update overall MyHospitals README.
        *   [ ] **Refine `data_dictionary_cancer.md`** based on full dataset analysis.
        *   [ ] Consider adding scripts/notebooks for data validation or example queries.
    *   [X] For AIHW MyHospitals - `MYH-LOS` (Length of Stay) measure category:
        *   [X] Identify next measure category.
        *   [X] Adapt/extend script.
        *   [X] Test data fetch (Completed with 2-page limit, ~2k records fetched, full dataset ~517k).
        *   [X] Create specific documentation.
        *   [X] Update overall MyHospitals README.
        *   [ ] **Refine `data_dictionary_los.md`** based on full dataset analysis if feasible.
        *   [ ] Consider adding scripts/notebooks for data validation or example queries.
    *   [X] For AIHW MyHospitals - `MYH-HH` (Hand Hygiene) measure category:
        *   [X] Identify next measure category.
        *   [X] Adapt/extend script.
        *   [X] Test data fetch (Completed with 2-page limit, ~2k records fetched, full dataset ~38k).
        *   [X] Create specific documentation.
        *   [X] Update overall MyHospitals README.
        *   [ ] **Refine `data_dictionary_hand_hygiene.md`** based on full dataset analysis if feasible.
        *   [ ] Consider adding scripts/notebooks for data validation or example queries.
    *   [ ] For further AIHW MyHospitals measure categories:
        *   [ ] Identify next measure category.
        *   [ ] Repeat process: test fetch, document, update logs.

### Documentation Refinement
*   [X] Flesh out `CONTRIBUTING.MD` with detailed guidelines.
*   [X] Develop `CODE_OF_CONDUCT.MD`.

## Future Tasks (To be detailed from Roadmap - Phase 2 & 3)
*   [ ] Expand Dataset Coverage (Stats NZ, ABS, other sources).
*   [ ] Develop General User Guides (`docs/guides/`).
*   [ ] Introduce Basic Data Access/Analysis Examples.
*   [ ] Community Building Initiatives.
*   [ ] Explore Advanced Features (Visualizations, Dashboards).
*   [ ] Long-Term Maintenance Planning.

---
*This TODO list will be updated regularly. Check the `ROADMAP.md` for broader project phases.*
