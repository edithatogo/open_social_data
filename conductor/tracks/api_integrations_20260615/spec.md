# Specification: Implement Agency API Providers (ABS, Stats NZ)

## 1. Overview
Implement concrete struct providers that satisfy the `DatasetProvider` trait:
- `AbsProvider` to interface with the Australian Bureau of Statistics (ABS) API endpoints.
- `StatsNzProvider` to interface with the Statistics New Zealand (Stats NZ) API endpoints.

## 2. Requirements
- **Endpoint Integration:** Setup async REST calls to ABS and Stats NZ APIs.
- **Payload Parsing:** Robustly parse returned XML/JSON schemas into standard data engine formats.
- **Paging:** Implement pagination mechanics (fetching records in chunks to prevent timeout).
- **Error Handling:** Utilize `thiserror` to define precise connection and status-code errors.

## 3. Style and Standards
Refer to [code-styleguides.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/code-styleguides.md) and [workflow.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/workflow.md).
