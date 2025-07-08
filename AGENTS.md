# AGENTS.md - Instructions for AI Contributors

This file provides guidelines for AI agents contributing to the Open Social Datasets repository. Adherence to these instructions will help maintain consistency, quality, and organization.

## Core Principles for AI Contributions:

1.  **Understand the Project's Philosophy:** Before making changes, review the main `README.md` to understand the project's mission of making social data accessible and understandable.
2.  **Follow Established Structure:**
    *   All new datasets must be placed under the appropriate subdirectory within `datasets/` (e.g., `datasets/stats_nz/` or `datasets/abs/`).
    *   Each dataset must have its own folder (e.g., `datasets/stats_nz/my_dataset_name/`).
    *   General documentation goes into `docs/guides/` or `docs/technical/`.
3.  **Use Markdown for Documentation:** All textual documentation files (`.md`) must use standard Markdown syntax. Ensure formatting is clean and readable.
4.  **Utilize Templates for Datasets:**
    *   When adding a new dataset, you **MUST** use the provided templates from the `/templates` directory:
        *   `templates/dataset_readme_template.md` (for the dataset's specific `README.md`)
        *   `templates/data_dictionary_template.md` (for `data_dictionary.md`)
        *   `templates/accessible_guide_template.md` (for `accessible_guide.md`)
    *   Fill these templates out as comprehensively as possible with accurate information.
5.  **Keep Documentation Clear and Accessible:**
    *   Write for a broad audience. Explain jargon or link to definitions.
    *   The `accessible_guide.md` is particularly important for non-technical users.
6.  **Update Changelog:** For any significant additions, new datasets, or structural changes, add an entry to `CHANGELOG.md` under the `[Unreleased]` section.
7.  **Verify Links and Information:** Double-check any external links (e.g., to source data, methodology pages) to ensure they are active and correct. Ensure metadata aligns with source information.
8.  **Code Contributions (if any):**
    *   If adding scripts (e.g., for data fetching or processing), ensure they are well-commented and placed in a `scripts/` subdirectory within the relevant dataset's folder.
    *   Mention any dependencies or setup required for scripts.
9.  **Session Logging:** When adding or significantly updating a dataset, consider creating a session log using `SESSION_LOG_TEMPLATE.md` and saving it within the dataset's directory (e.g., `datasets/stats_nz/my_dataset_name/SESSION_LOG_YYYY-MM-DD.md`). This helps with transparency and reproducibility.
10. **Commit Messages:** Use clear and descriptive commit messages. If addressing a specific item from `TODO.md`, reference it.

## Specific Instructions:

*   **Do not directly commit large data files.** Provide links or download scripts. Small example snippets are acceptable if clearly indicated.
*   **Respect original source licenses.** Always document the license of the original data.
*   **If unsure, ask for clarification.** It's better to ask than to implement incorrectly.

By following these guidelines, AI agents can significantly contribute to the quality and utility of this repository.
