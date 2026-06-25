# Specification: Data Quality Assertions and Delta Updates

## 1. Overview
Implement data validation constraints on incoming datasets prior to Parquet conversion and develop a delta logic flow that detects new segments or changes dynamically to fetch updates incrementally.

## 2. Requirements
- **Validation Engine:** Provide runtime check macros/helpers (e.g. column value ranges, null-value limits, unique keys).
- **Delta/Incremental Loads:** Use HTTP headers (e.g., `If-Modified-Since`, ETag) and track download state history to fetch additions instead of replacing existing outputs.
- **Reporting:** Provide a clean validation report summarizing quality runs (e.g., number of rows validated, passing assertions, failed constraints).

## 3. Style and Standards
Refer to [code-styleguides.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/code-styleguides.md) and [workflow.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/workflow.md).
