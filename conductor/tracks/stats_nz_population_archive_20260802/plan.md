# Track Plan: Stats NZ population exact-edition archive

## Phase 1: Freeze and validate the source

- [x] Select the exact official Stats NZ 30 June 2025 provisional edition.
- [x] Sample the complete public workbook and record its byte length and SHA-256.
- [x] Define fail-closed URL, media-type, XLSX, sheet and edition-marker checks.

## Phase 2: Implement hosted acquisition

- [x] Add the source-specific packet builder and hermetic contract tests.
- [x] Route the approved source through the existing GitHub Actions to Hugging Face workflow.
- [x] Update the public dataset card, repository TODO and changelog.

## Phase 3: Publish and qualify evidence

- [ ] Merge the implementation after hosted checks pass.
- [ ] Execute the exact-edition hosted capture and verify the Hub packet and receipt.
- [ ] Record immutable revisions and digests, run agent-panel qualification and archive the track.
