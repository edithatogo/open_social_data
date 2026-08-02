# Track Plan: Stats NZ Meshblock 2026 hosted archive

## Phase 1: Contract and failure semantics

- [x] Pin the public ArcGIS item, layer, expected modification timestamp and feature count (`0c4dc31`).
- [x] Add hermetic tests for URL boundaries, deterministic compression, inventories and pages (`0c4dc31`).
- [x] Implement fail-closed metadata, ID, page and source-stability validation (`0c4dc31`, `94f67a9`).

## Phase 2: Hosted acquisition and preservation

- [x] Add a manually dispatched GitHub Actions acquisition workflow (`0c4dc31`).
- [x] Add a source-specific Hugging Face dataset card and protected-secret boundary (`0c4dc31`, `728c4d6`).
- [x] Execute the full hosted capture and verify all 57,575 feature IDs and checksums ([run 30750165664](https://github.com/edithatogo/open_social_data/actions/runs/30750165664)).
- [x] Upload the packet and receipt to immutable Hugging Face revisions (`3f2dc0a4d95a4fcb495551098d58fc5bce9c9202`, `34c093646f884d7b57447231d6605e83739bb302`).

## Phase 3: Downstream evidence

- [x] Record the GitHub run, Hugging Face revisions and manifest SHA-256 in this index (`c07f401`).
- [x] Update open_social_data issue 35 and the affected RIOPA Conductor evidence indexes (`c07f401`; downstream `5d2ca1a`).
- [x] Run three-member agent-panel qualification without broadening the track's claims ([qualification report](./qualification-report.md)).
- [ ] Archive the validated track and preserve registry continuity.

## Review fixes

- [x] Reject redirects to non-approved response URLs (`ee9bda0`).
- [x] Restrict `HF_TOKEN` to publication steps (`ee9bda0`).
