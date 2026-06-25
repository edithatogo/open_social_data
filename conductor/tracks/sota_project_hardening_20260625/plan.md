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

- [ ] Review reqwest compression and streaming support.
- [ ] Review Polars lazy, time/date, CSV/JSON IO, and SQL support.
- [ ] Review rusqlite catalog backup, JSON, limits, tracing, or extension support.
- [ ] Review CLI completions, man pages, and structured reference generation.
- [ ] Record accepted and rejected feature decisions with rationale.

## Phase 4: CLI Polish and Generated Reference

- [ ] Add shell completion generation.
- [ ] Add man-page generation or equivalent CLI reference output.
- [ ] Add machine-readable CLI help/reference export if useful for docs.
- [ ] Add tests for generated CLI metadata where feasible.

## Phase 5: CI, Security, and Release Hardening

- [ ] Add or document cargo nextest.
- [ ] Add cargo audit or equivalent advisory checks.
- [ ] Add cargo deny license/source policy checks.
- [ ] Add cargo llvm-cov or equivalent coverage reporting.
- [ ] Add docs build validation.
- [ ] Add release dry-run checks and no-live-publish guardrails.
- [ ] Add SBOM/provenance guidance or generation scripts.

## Phase 6: Parser Robustness and Performance

- [ ] Add compact ABS SDMX and Stats NZ OData fixture payloads.
- [ ] Add snapshot or regression tests for parsed row output.
- [ ] Add parser benchmarks for representative fixture sizes.
- [ ] Add property or fuzz coverage where it provides meaningful parser safety.
- [ ] Document fixture sourcing, minimisation, and licence constraints.

## Phase 7: Review and Close

- [ ] Run Rust checks, tests, docs build, dependency policy checks, and parser quality gates.
- [ ] Update ROADMAP.md, TODO.md, CHANGELOG.md, and Conductor metadata.
- [ ] Review whether any optional experimental feature should remain gated or be reverted.
- [ ] Commit and mark Track 12 complete only after validation evidence is recorded.
