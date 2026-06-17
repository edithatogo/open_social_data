# Plan: Short-term Completion and Source Validation

All tasks require Git commits upon completion. Pushes and reviews should occur at the end of the track.

## Phase 1: Status Alignment

- [x] Task: Audit `TODO.md`, `ROADMAP.md`, `CHANGELOG.md`, and `conductor/tracks.md` for stale Phase 1 status.
- [x] Task: Update status files so completed foundation and dataset-pack work is accurately reflected.
- [x] Task: Convert any old "on hold" notes into dated blockers or explicit follow-up tasks.

## Phase 2: Dataset Pack Completeness

- [x] Task: Verify every current Stats NZ, ABS, and AIHW dataset pack has required documentation and scripts.
- [x] Task: Add missing session logs, data dictionary notes, or accessible guides where gaps are found.
- [x] Task: Document official source access paths and expected credentials/API endpoint requirements.

## Phase 3: Local Validation

- [x] Task: Add or update a release-readiness checklist for local validation.
- [x] Task: Run Python script compile checks for dataset wrappers.
- [x] Task: Run AIHW local Parquet validation and example queries.
- [x] Task: Run Rust validation using `CARGO_TARGET_DIR=C:\tmp\open_social_data_target2`.
- [x] Task: Record validation evidence in the track notes or changelog.

## Phase 4: QBIS Decision

- [x] Task: Evaluate whether QBIS can be fetched through the current ABS provider.
- [x] Task: Implement the QBIS wrapper if feasible, or defer with a dated blocker and current source-link evidence.
- [x] Task: Update QBIS README/data dictionary with the final decision.

## Phase 5: Review and Close

- [x] Task: Run `git diff --check`.
- [x] Task: Commit the short-term completion slice.
- [x] Task: Conductor - Review track completion and mark status accordingly.
