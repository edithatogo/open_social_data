# Plan: Medium-term Dataset Expansion and Examples

All tasks require Git commits upon completion. Pushes and reviews should occur at the end of each phase.

## Phase 1: Dataset Expansion Backlog

- [x] Task: Identify candidate Stats NZ datasets beyond population, CPI, and labour market.
- [x] Task: Identify candidate ABS datasets beyond QBIS, CPI, Labour Force, and National Accounts.
- [x] Task: Identify one or more additional source agencies and assess licence/API feasibility.
- [x] Task: Prioritize candidates by social value, source stability, licence clarity, and fetch feasibility.

## Phase 2: Dataset Pack Implementation

- [x] Task: Implement at least three additional dataset packs with README, data dictionary, accessible guide, script, and session log.
- [x] Task: Add source metadata capture for codelists, units, update cadence, methodology, caveats, and licences.
- [x] Task: Add validation checks or documented manual checks for each implemented pack.

## Phase 3: General User Guides

- [x] Task: Create `docs/guides/understanding-social-statistics-concepts.md`.
- [x] Task: Create `docs/guides/interpreting-common-visualizations.md`.
- [x] Task: Create `docs/guides/ethical-use-of-social-data.md`.
- [x] Task: Link guides from the main README and dataset packs where relevant.

## Phase 4: Examples

- [x] Task: Add Rust CLI examples for reading, filtering, and summarising selected Parquet/source metadata outputs.
- [x] Task: Retire R example capability until an R runtime is part of local validation.
- [x] Task: Include expected output notes or lightweight tests for examples.
- [x] Task: Keep examples small enough for local execution.

## Phase 5: Review and Close

- [x] Task: Run script compilation/format checks.
- [x] Task: Run Rust CLI example commands against available local data when local build/link resources permit.
- [x] Task: Update `TODO.md`, `CHANGELOG.md`, and `ROADMAP.md`.
- [x] Task: Complete Rust CLI run/test validation when local disk permits.
- [x] Task: Commit the medium-term expansion slice. (`21538e2`)
- [x] Task: Conductor - Review track completion and mark status accordingly.
