# Track Plan: Stats NZ Meshblock 2026 hosted archive

## Phase 1: Contract and failure semantics

- [x] Pin the public ArcGIS item, layer, expected modification timestamp and feature count.
- [x] Add hermetic tests for URL boundaries, deterministic compression, inventories and pages.
- [x] Implement fail-closed metadata, ID, page and source-stability validation.

## Phase 2: Hosted acquisition and preservation

- [x] Add a manually dispatched GitHub Actions acquisition workflow.
- [x] Add a source-specific Hugging Face dataset card and protected-secret boundary.
- [x] Execute the full hosted capture and verify all 57,575 feature IDs and checksums.
- [x] Upload the packet and receipt to immutable Hugging Face revisions.

## Phase 3: Downstream evidence

- [x] Record the GitHub run, Hugging Face revisions and manifest SHA-256 in this index.
- [x] Update open_social_data issue 35 and the affected RIOPA Conductor evidence indexes.
- [ ] Run agent-panel qualification and archive this track without broadening its claims.
