# Project TODO List

This file tracks specific, actionable tasks for the Open Social Datasets project. Items will be moved from the `ROADMAP.md` to here as they become current.

## Phase 1: Foundation & Initial Datasets

### Core Setup (Most items completed, see CHANGELOG)
*   [X] Define Project Philosophy and Goals.
*   [X] Establish Repository Structure.
*   [X] Develop Core Documentation Templates (`dataset_readme_template.md`, `data_dictionary_template.md`, `accessible_guide_template.md`).
*   [X] Write Initial Content for `README.md`, `CHANGELOG.md`, `ROADMAP.md`, `TODO.md`, `SESSION_LOG_TEMPLATE.md`.
*   [ ] **Create `AGENTS.md`:** Add initial instructions for AI agent contributions to the root directory.

### Initial Dataset Population
*   **Stats NZ Datasets:**
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
    *   [ ] Identify and prioritize dataset(s) from the Australian Institute of Health and Welfare (AIHW) (Pending user instruction).
    *   [ ] For Dataset 1 (AIHW):
        *   [ ] Create directory structure.
        *   [ ] Populate documentation (README, Data Dictionary, Accessible Guide).
        *   [ ] Develop data access/processing scripts.
        *   [ ] Log session.

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
