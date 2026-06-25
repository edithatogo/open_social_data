# Plan: SOTA Project Hardening and Documentation Platform

All tasks require validation evidence before completion. Commits should be split by phase where practical.

## Phase 1: Dependency Automation

- [x] Add renovate.json for Cargo, GitHub Actions, npm/docs tooling, and lockfile maintenance.
- [x] Configure dependency dashboard and grouped patch/minor updates.
- [x] Keep major Rust data-stack upgrades manual.
- [x] Document Renovate triage expectations in maintainer docs.

## Phase 2: Astro 7 and Starlight Documentation

- [x] Scaffold an Astro 7/Starlight docs site.
- [x] Add installation, quickstart, and CLI command reference pages.
- [x] Add provider authoring, dataset pack, catalog, validation, and roadmap-status pages.
- [x] Add release, provenance, and archival docs pages from existing guidance.
- [x] Add a local docs build command and CI/docs validation gate.

## Phase 3: Rust Dependency Feature Review

- [x] Review reqwest compression and streaming support.
- [x] Review Polars lazy, time/date, CSV/JSON IO, and SQL support.
- [x] Review rusqlite catalog backup, JSON, limits, tracing, or extension support.
- [x] Review CLI completions, man pages, and structured reference generation.
- [x] Record accepted and rejected feature decisions with rationale.

## Phase 4: CLI Polish and Generated Reference

- [x] Add shell completion generation.
- [x] Add man-page generation or equivalent CLI reference output.
- [x] Add machine-readable CLI help/reference export if useful for docs.
- [x] Add tests for generated CLI metadata where feasible.

## Phase 5: CI, Security, and Release Hardening

- [x] Add or document cargo nextest.
- [x] Add cargo audit or equivalent advisory checks.
- [x] Add cargo deny license/source policy checks.
- [x] Add cargo llvm-cov or equivalent coverage reporting.
- [x] Add docs build validation.
- [x] Add release dry-run checks and no-live-publish guardrails.
- [x] Add SBOM/provenance guidance or generation scripts.

## Phase 6: Parser Robustness and Performance

- [x] Add compact ABS SDMX and Stats NZ OData fixture payloads.
- [x] Add snapshot or regression tests for parsed row output.
- [x] Add parser benchmarks for representative fixture sizes.
- [x] Add property or fuzz coverage where it provides meaningful parser safety.
- [x] Document fixture sourcing, minimisation, and licence constraints.

## Phase 7: Review and Close

- [ ] Run Rust checks, tests, docs build, dependency policy checks, and parser quality gates.
- [ ] Update ROADMAP.md, TODO.md, CHANGELOG.md, and Conductor metadata.
- [ ] Review whether any optional experimental feature should remain gated or be reverted.
- [ ] Commit and mark Track 12 complete only after validation evidence is recorded.
