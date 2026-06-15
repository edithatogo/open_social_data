# Specification: Local Metadata Caching and SQLite/DuckDB Cataloging

## 1. Overview
Implement a local caching engine using an embedded SQLite or DuckDB database to catalog available agency datasets, tables, versions, and local cache paths. This avoids redundant HTTP API queries when exploring statistical catalogues.

## 2. Requirements
- **Local DB Integration:** Integrate `rusqlite` or `duckdb` crate to maintain catalog schema.
- **Auto-Sync:** Onboard an automated catalog syncer that pulls agency catalogs and populates the database schema.
- **Search API:** Provide methods to query metadata (e.g. search datasets by keyword, name, or code).

## 3. Style and Standards
Refer to [code-styleguides.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/code-styleguides.md) and [workflow.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/workflow.md).
