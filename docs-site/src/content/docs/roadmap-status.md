---
title: Roadmap and Track Status
description: Current project track status, how to read Conductor tracks, archived tracks 1–11 overview, and the improvement backlog.
---

# Roadmap and Track Status

The project uses **Conductor tracks** to organise work into bounded, reviewable slices. Each track lives in `conductor/tracks/` with a plan, spec, metadata, and validation evidence.

## Current Track: Track 12 (In Progress)

**Track:** SOTA Project Hardening and Documentation Platform  
**Directory:** `conductor/tracks/sota_project_hardening_20260625/`

### Goals

- Add Renovate dependency automation for Cargo, GitHub Actions, and npm/docs tooling
- Scaffold Astro 7/Starlight documentation site (the site you are reading now)
- Review optional Rust crate features and CLI reference generation
- Add CI, security, coverage, parser benchmark/fuzz, SBOM, and release dry-run gates
- Add parser fixtures, regression tests, and benchmarks for ABS SDMX and Stats NZ OData parsing

### Phase status

| Phase | Status |
|-------|--------|
| Phase 1: Dependency Automation | ✅ Complete (Renovate configured) |
| Phase 2: Astro 7/Starlight Documentation | 🔄 In progress |
| Phase 3: Rust Dependency Feature Review | ⬜ Pending |
| Phase 4: CLI Polish and Generated Reference | ⬜ Pending |
| Phase 5: CI, Security, and Release Hardening | ⬜ Pending |
| Phase 6: Parser Robustness and Performance | ⬜ Pending |
| Phase 7: Review and Close | ⬜ Pending |

### Done When

- Dependency updates are automated but major upgrades remain explicit maintainer decisions
- Documentation can be built locally and validated in CI
- Optional feature choices are documented with accepted/rejected rationale
- CLI completions, man pages, or equivalent generated reference material are available
- Parser quality and performance gates protect the core row-level ingestion paths

---

## How to Read Conductor Tracks

Each track directory contains:

```
conductor/tracks/<track_name>/
  metadata.json    # Track metadata: ID, status, dates, owner
  plan.md          # Detailed plan with phases and checkboxes
  spec.md          # Specification and requirements
  validation.md    # Validation evidence and gate records
```

Archived tracks live under `conductor/tracks/archived/`.

The master index is at `conductor/tracks.md`, which lists every track with its status and a link to its directory.

### Track status conventions

| Status | Meaning |
|--------|---------|
| `[x]` | Track is complete and archived |
| `[ ]` | Track is in progress or pending |
| `(archived)` | Track directory is under `tracks/archived/` |

---

## Archived Tracks 1–11

All foundational tracks are complete and archived.

| Track | Name | Summary |
|-------|------|---------|
| 1 | Rust Core Scaffold | Define `DatasetProvider` trait, module structure, error types, and basic project scaffolding |
| 2 | Agency API Providers | Implement `AbsProvider` (SDMX-JSON) and `StatsNzProvider` (OData) with mocked HTTP test support |
| 3 | Data Pipeline & Parquet | Schema validation, `RecordBatchBuilder`, `write_parquet_atomic`, and `read_parquet` helpers |
| 4 | CLI Development | `list`, `status`, `fetch`, `catalog list/search/sync` via `clap`, with `indicatif` progress bars |
| 5 | Hardening & Optimisation | Retry policy with exponential backoff, circuit breaker pattern, panic-safe provider execution |
| 6 | Data Quality | `QualityAssertion` enum, `QualityReport` with atomic persistence, `DeltaUpdater` for incremental Parquet |
| 7 | Documentation & Registry | Module-level rustdoc, general data-access guides, GitHub Actions CI, Cargo metadata |
| 8 | Local Catalog | JSON-backed and SQLite-backed local catalogs with search, sync, and atomic persistence |
| 9 | Short-term Completion | Align status files, close stale blockers, add release-readiness validation |
| 10 | Medium-term Expansion | Expand dataset coverage (Stats NZ, ABS, AIHW), add guides and Rust CLI examples |
| 11 | Long-term Sustainability | Maintenance checks, contribution workflow, dashboard prototype, release/provenance practices |

### Key achievements from completed tracks

- **Dataset packs:** 5 ABS packs (AWE, CPI, Labour Force, National Accounts, QBIS), 4 Stats NZ packs (CPI, Population Estimates, Labour Market, Household Income), 6 AIHW MyHospitals measure packs, 1 MoH NZ Health Survey pack
- **Rust library:** ~4,000+ lines of core library code with 90%+ test coverage on provider and catalog modules
- **Data pipeline:** Schema-validated, atomic Parquet export with quality assertions at every fetch
- **HTTP hardening:** Retry with exponential backoff, circuit breaker, panic isolation, conditional request support (ETag/`Last-Modified`)

---

## Improvement Backlog

The Conductor improvement backlog at `conductor/improvement-backlog.md` tracks candidate improvements that are not yet assigned to a track:

- Validate `conductor/learning-log.md` entries against schema in CI
- Add repository-scoped script to append learning candidates
- Capture registry/review/skills-feedback events into the backlog
- Add phase-level retrospective notes for each Phase 1/2/3/4 run

---

## Source Material

- [ROADMAP.md](https://github.com/open-social-data/open-social-data-core/blob/main/ROADMAP.md) — High-level project roadmap with short, medium, and long-term horizons
- [conductor/tracks.md](https://github.com/open-social-data/open-social-data-core/tree/main/conductor/tracks.md) — Master track index
- [conductor/improvement-backlog.md](https://github.com/open-social-data/open-social-data-core/tree/main/conductor/improvement-backlog.md) — Improvement backlog
- [docs/technical/release_readiness_checklist.md](https://github.com/open-social-data/open-social-data-core/tree/main/docs/technical/release_readiness_checklist.md) — Release readiness checklist