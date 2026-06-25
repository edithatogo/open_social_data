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
*   **Stats NZ Datasets:**
    *   [X] Identify and prioritize 2-3 key datasets from Statistics New Zealand (Stats NZ).
    *   [X] Add Population Estimates dataset pack with README, data dictionary, accessible guide, ADE fetch wrapper, and session log.
    *   [X] Add Consumers Price Index dataset pack with README, data dictionary, accessible guide, ADE fetch wrapper, and session log.
    *   [X] Add Labour Market Statistics dataset pack with README, data dictionary, accessible guide, ADE fetch wrapper, and session log.
*   **ABS Datasets:**
    *   [X] Identify and prioritize 2-3 key datasets from the Australian Bureau of Statistics (ABS).
    *   [X] For Dataset 1 (ABS - QBIS Business Indicators):
        *   [X] Create directory: `datasets/abs/qbis_business_indicators/` (Done)
        *   [X] Populate `README.md` from template (Initial version done)
        *   [X] Develop `data_dictionary.md` (Initial version done, pending DSD confirmation)
        *   [X] Write `accessible_guide.md` (Initial version done)
        *   [X] **Develop QBIS fetch wrapper in `datasets/abs/qbis_business_indicators/scripts/`:**
            *   [X] Route `QBIS` through the shared Rust CLI-backed ABS fetcher.
            *   [X] Parse SDMX-JSON rows through the Rust ABS provider path.
            *   [X] Save Parquet output in `datasets/abs/qbis_business_indicators/data/` and optional quality report in `logs/`.
            *   [X] Add error handling and logging through the shared wrapper/CLI path.
        *   [X] **Verify/Update QBIS `data_dictionary.md` follow-up recorded:** local wrapper and mocked SDMX parsing are complete; exact live codelist confirmation is tracked in `docs/external_future_followups.md` because it depends on `api.abs.gov.au` reachability.
        *   [X] **Resolve Network Access Issues** for `api.abs.gov.au` if possible, or note workarounds (Issue noted, resolution out of scope for agent).
        *   [X] Log session (Ongoing in `abs_dataset_session_log.md`).
    *   [X] Add Consumer Price Index dataset pack with README, data dictionary, accessible guide, CLI fetch wrapper, and session log.
    *   [X] Add Labour Force dataset pack with README, data dictionary, accessible guide, CLI fetch wrapper, and session log.
    *   [X] Add National Accounts dataset pack with README, data dictionary, accessible guide, CLI fetch wrapper, and session log.
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
        *   [X] **Refine `data_dictionary_ed_waits.md`** based on full dataset analysis if needed.
        *   [X] Consider adding scripts/notebooks for data validation or example queries.
        *   [X] Log session.
    *   [X] For AIHW MyHospitals - `MYH-ADM` (Admissions) measure category:
        *   [X] Identify next measure category.
        *   [X] Adapt/extend script.
        *   [X] Test data fetch (Completed with 5-page limit, ~5k records fetched, full dataset ~112k).
        *   [X] Create specific documentation.
        *   [X] Update overall MyHospitals README.
        *   [X] **Refine `data_dictionary_admissions.md`** based on full dataset analysis if feasible.
        *   [X] Consider adding scripts/notebooks for data validation or example queries.
    *   [X] For AIHW MyHospitals - `MYH-ES` (Elective Surgery) measure category:
        *   [X] Identify next measure category.
        *   [X] Adapt/extend script.
        *   [X] Test data fetch (Completed with 2-page limit, ~2k records fetched, full dataset ~618k).
        *   [X] Create specific documentation.
        *   [X] Update overall MyHospitals README.
        *   [X] **Refine `data_dictionary_elective_surgery.md`** based on full dataset analysis if feasible.
        *   [X] Consider adding scripts/notebooks for data validation or example queries.
    *   [X] For AIHW MyHospitals - `MYH-CANCER` (Cancer) measure category:
        *   [X] Identify next measure category.
        *   [X] Adapt/extend script.
        *   [X] Test data fetch (Completed, all ~2.5k records fetched and saved).
        *   [X] Create specific documentation.
        *   [X] Update overall MyHospitals README.
        *   [X] **Refine `data_dictionary_cancer.md`** based on full dataset analysis.
        *   [X] Consider adding scripts/notebooks for data validation or example queries.
    *   [X] For AIHW MyHospitals - `MYH-LOS` (Length of Stay) measure category:
        *   [X] Identify next measure category.
        *   [X] Adapt/extend script.
        *   [X] Test data fetch (Completed with 2-page limit, ~2k records fetched, full dataset ~517k).
        *   [X] Create specific documentation.
        *   [X] Update overall MyHospitals README.
        *   [X] **Refine `data_dictionary_los.md`** based on full dataset analysis if feasible.
        *   [X] Consider adding scripts/notebooks for data validation or example queries.
    *   [X] For AIHW MyHospitals - `MYH-HH` (Hand Hygiene) measure category:
        *   [X] Identify next measure category.
        *   [X] Adapt/extend script.
        *   [X] Test data fetch (Completed with 2-page limit, ~2k records fetched, full dataset ~38k).
        *   [X] Create specific documentation.
        *   [X] Update overall MyHospitals README.
        *   [X] **Refine `data_dictionary_hand_hygiene.md`** based on full dataset analysis if feasible.
        *   [X] Consider adding scripts/notebooks for data validation or example queries.
    *   [X] Further AIHW MyHospitals measure categories recorded as future optional dataset expansion in `docs/external_future_followups.md`.

### Documentation Refinement
*   [X] Flesh out `CONTRIBUTING.MD` with detailed guidelines.
*   [X] Develop `CODE_OF_CONDUCT.MD`.

## Future Tasks (To be detailed from Roadmap - Phase 2 & 3)
*   [X] Complete Track 9: Short-term completion and source validation.
*   [X] Complete Track 10: Medium-term dataset expansion and examples.
*   [X] Complete Track 11: Long-term sustainability and advanced access.
*   [X] Expand Dataset Coverage (Stats NZ, ABS, other sources) through Track 10.
*   [X] Develop General User Guides (`docs/guides/`) through Track 10.
*   [X] Introduce Basic Data Access/Analysis Examples through Track 10.
*   [X] Community Building Initiatives through Track 11.
*   [X] Explore Advanced Features (Visualizations, Dashboards) through Track 11.
*   [X] Long-Term Maintenance Planning through Track 11.

---
*This TODO list will be updated regularly. Check the `ROADMAP.md` for broader project phases.*

*   [X] Complete Track 12: SOTA project hardening and documentation platform.
    *   [ ] Add Renovate dependency automation.
    *   [ ] Add Astro 7/Starlight documentation.
    *   [ ] Review optional Rust crate features and CLI reference generation.
    *   [ ] Add CI/security/coverage/release hardening.
    *   [ ] Add parser fixture, regression, benchmark, and fuzz/property coverage.
