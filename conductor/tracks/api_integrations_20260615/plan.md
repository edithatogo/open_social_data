# Plan: Implement Agency API Providers (ABS, Stats NZ)

All tasks require Git commits upon completion. Pushes and reviews must occur at the end of each phase.

## Phase 1: ABS Provider Integration
- [x] Task: Create `AbsProvider` type structure and implement URL builder module
- [x] Task: Implement async HTTP download logic and page streaming for ABS records
- [ ] Task: Write mock and integration tests for ABS fetching code
- [ ] Task: Conductor - Push changes, perform peer review of ABS implementation (Protocol in workflow.md)

## Phase 2: Stats NZ Provider Integration
- [x] Task: Create `StatsNzProvider` type structure and implement OData parser module
- [x] Task: Implement OData pagination and HTTP client mapping for Stats NZ datasets
- [ ] Task: Write mock and integration tests for Stats NZ fetching code
- [ ] Task: Conductor - Push changes, perform peer review of Stats NZ implementation (Protocol in workflow.md)

## Swarm Notes - 2026-06-15
- Added provider modules for ABS and Stats NZ plus URL-building unit tests.
- Live HTTP integration tests remain open until the local Rust toolchain can link build scripts.
- Provider fetches now apply cached `If-None-Match` / `If-Modified-Since` validators through `FetchOptions`, capture response `ETag` / `Last-Modified`, return `FetchResult::NotModified` for HTTP 304, and return `FetchResult::Fetched` for frame-bearing fetches.
- Refined `FetchResult` into explicit `Fetched` and `NotModified` variants to avoid inconsistent frame/not-modified states, with compatibility helpers for validators and frame extraction.
