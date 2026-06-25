# Specification: SOTA Project Hardening and Documentation Platform

## Overview

Advance Open Social Datasets from roadmap-complete to public-project mature with dependency automation, documentation infrastructure, CI/release hygiene, and parser quality tooling.

## Scope

- Renovate dependency automation for Cargo, GitHub Actions, npm/docs tooling, and lockfile maintenance.
- Astro 7 plus Starlight documentation site scaffolding.
- Rust crate feature review for provider fetches, Polars processing, SQLite catalogs, and CLI reference generation.
- CLI polish for shell completions, man pages, and machine-readable command reference output.
- CI hardening with nextest, audit/advisory checks, deny policy, coverage, docs build, and release dry-run checks.
- Parser robustness and performance work for ABS SDMX and Stats NZ OData row-level parsing.
- Release packaging, provenance, SBOM, and no-live-publish guardrails.

## Requirements

- Dependency automation must be conservative; major upgrades remain explicit maintainer decisions.
- Documentation must be generated or checked from source where feasible.
- Experimental crate features must be gated and justified.
- Parser benchmark, fuzz, or fixture suites must avoid large public-data payloads.
- Release and provenance work must not publish or upload artifacts without explicit approval.

## Acceptance Criteria

- renovate.json covers Cargo, GitHub Actions, docs packages, lockfile maintenance, dependency dashboard, and grouped updates.
- Astro 7/Starlight docs cover installation, CLI usage, providers, dataset packs, catalogs, validation, roadmap status, and release process.
- Optional dependency features are reviewed, implemented where useful, and documented when rejected.
- CLI completions and man pages exist, or a clear deferral is recorded.
- CI or local equivalents cover fmt, clippy, tests, nextest, audit, deny, coverage, docs build, and release dry-run checks.
- ABS SDMX and Stats NZ OData parsers have compact fixtures plus regression, benchmark, fuzz, or property coverage.
- ROADMAP.md, TODO.md, CHANGELOG.md, and conductor/tracks.md agree on track status.

## Standards

Follow AGENTS.md, CONTRIBUTING.md, conductor/code-styleguides.md, and conductor/workflow.md. Prefer Rust-first user-facing workflows.
