# Plan: Implement Agency API Providers (ABS, Stats NZ)

All tasks require Git commits upon completion. Pushes and reviews must occur at the end of each phase.

## Phase 1: ABS Provider Integration
- [x] Task: Create `AbsProvider` type structure and implement URL builder module
- [x] Task: Implement async HTTP download logic and page streaming for ABS records
- [x] Task: Write mock and integration tests for ABS fetching code
- [x] Task: Conductor - Review ABS implementation against current provider tests

## Phase 2: Stats NZ Provider Integration
- [x] Task: Create `StatsNzProvider` type structure and implement OData parser module
- [x] Task: Implement OData pagination and HTTP client mapping for Stats NZ datasets
- [x] Task: Write mock and integration tests for Stats NZ fetching code
- [x] Task: Conductor - Review Stats NZ implementation against current provider tests

## Swarm Notes - 2026-06-15
- Added provider modules for ABS and Stats NZ plus URL-building unit tests.
- Local Rust validation now links and passes. Live HTTP endpoint checks remain an external-source follow-up rather than a track blocker.
- Provider fetches now apply cached `If-None-Match` / `If-Modified-Since` validators through `FetchOptions`, capture response `ETag` / `Last-Modified`, return `FetchResult::NotModified` for HTTP 304, and return `FetchResult::Fetched` for frame-bearing fetches.
- Refined `FetchResult` into explicit `Fetched` and `NotModified` variants to avoid inconsistent frame/not-modified states, with compatibility helpers for validators and frame extraction.
- Added dependency-free mocked HTTP source tests for ABS and Stats NZ conditional headers and HTTP 304 `FetchResult::NotModified` behavior.
