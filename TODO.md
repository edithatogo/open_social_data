# Project TODO List

This file tracks specific, actionable tasks for the Open Social Datasets project. Items will be moved from the `ROADMAP.md` to here as they become current.

## Phase 1: Foundation & Initial Datasets

### Core Setup (Most items completed, see CHANGELOG)
*   [X] Define Project Philosophy and Goals.
*   [X] Establish Repository Structure.
*   [X] Develop Core Documentation Templates (`dataset_readme_template.md`, `data_dictionary_template.md`, `accessible_guide_template.md`).
*   [X] Write Initial Content for `README.md`, `CHANGELOG.md`, `ROADMAP.md`, `TODO.md`, `SESSION_LOG_TEMPLATE.md`.
*   [X] **Create `AGENTS.md`:** Add initial instructions for AI agent contributions to the root directory. (Done)

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
    *   [X] For Dataset 1 (AIHW - MyHospitals API, `MYH-ED-WAITS` measure category):
        *   [X] Create directory structure: `datasets/aihw/myhospitals/` (Done).
        *   [X] Explore API documentation (Swagger spec successfully analyzed).
        *   [X] Perform initial API interaction (Fetched measure categories successfully).
        *   [X] Select specific endpoint for data fetching (`/flat-formatted-data-extract/MYH-ED-WAITS`).
        *   [X] Develop `fetch_aihw_myhospitals_data.py` script.
        *   [X] Populate initial documentation for `MYH-ED-WAITS`.
        *   [X] **Test full run of `fetch_aihw_myhospitals_data.py`** for `MYH-ED-WAITS` (Completed, ~41k records fetched and saved).
        *   [ ] **Refine `data_dictionary_ed_waits.md`** based on full dataset analysis if needed.
        *   [ ] Consider adding scripts/notebooks for data validation or example queries for `MYH-ED-WAITS`.
        *   [X] Log session (Ongoing in `logs/aihw_myhospitals_session_log.md`).
    *   [X] For AIHW MyHospitals - `MYH-ADM` (Admissions) measure category:
        *   [X] Identify next measure category/dataset (`MYH-ADM` selected by user).
        *   [X] Adapt/extend `fetch_aihw_myhospitals_data.py` (Confirmed suitability, modified target category).
        *   [X] **Test data fetch for `MYH-ADM`** (Completed with 5-page limit, ~5k records fetched and saved, full dataset ~112k records).
        *   [X] Create specific documentation (`data_dictionary_admissions.md`, `accessible_guide_admissions.md`).
        *   [X] Update overall MyHospitals README.
        *   [ ] **Refine `data_dictionary_admissions.md`** based on full dataset analysis if feasible (or sample if full fetch is too long for sandbox).
        *   [ ] Consider adding scripts/notebooks for data validation or example queries for `MYH-ADM`.
    *   [ ] For further AIHW MyHospitals measure categories:
        *   [ ] Identify next measure category (e.g., `MYH-ES` Elective Surgery, `MYH-CANCER` Cancer).
        *   [ ] Repeat process: test fetch, document, update logs.

### Documentation Refinement
*   [ ] Flesh out `CONTRIBUTING.MD` with detailed guidelines.
*   [ ] Develop `CODE_OF_CONDUCT.MD`.

## Future Tasks (To be detailed from Roadmap - Phase 2 & 3)
*   [ ] Expand Dataset Coverage (Stats NZ, ABS, other sources).
*   [ ] Develop General User Guides (`docs/guides/`).
*   [ ] Introduce Basic Data Access/Analysis Examples.
*   [ ] Community Building Initiatives.
*   [ ] Explore Advanced Features (Visualizations, Dashboards).
*   [ ] Long-Term Maintenance Planning.

---
*This TODO list will be updated regularly. Check the `ROADMAP.md` for broader project phases.*
