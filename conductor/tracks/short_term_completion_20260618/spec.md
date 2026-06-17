# Specification: Short-term Completion and Source Validation

## 1. Overview

Complete the repository's first release candidate by aligning project status, validating current dataset packs, and converting stale blockers into explicit follow-up decisions.

## 2. Scope

This track covers short-term work after the completed foundation tracks:

- Roadmap, TODO, changelog, and Conductor status alignment.
- Dataset-pack validation for existing Stats NZ, ABS, and AIHW packs.
- QBIS-specific decisioning: implement the wrapper if feasible under the current provider, or defer with a documented external blocker.
- Local release-readiness checks for Rust, Python, docs, and data artefacts.
- Documentation of live-access blockers and source API migration risks.

## 3. Requirements

- Every current dataset pack has a README, data dictionary, accessible guide, access script, and session log.
- Every local validation command can run without live credentials unless explicitly marked as live-only.
- Existing AIHW Parquet validation remains runnable and documented.
- ABS and Stats NZ fetch paths document current official source access methods and expected credentials/API endpoint requirements.
- `TODO.md`, `ROADMAP.md`, `CHANGELOG.md`, and `conductor/tracks.md` are internally consistent.

## 4. Acceptance Criteria

- A release-readiness checklist exists and all local checks pass.
- No stale "in progress" Phase 1 roadmap language remains.
- QBIS has either a tested fetch wrapper or a dated deferral note with the precise blocker.
- The working tree is clean after implementation and validation.

## 5. Style and Standards

Follow `conductor/code-styleguides.md`, `conductor/workflow.md`, `CONTRIBUTING.md`, and `AGENTS.md`.
