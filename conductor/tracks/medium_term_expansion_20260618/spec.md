# Specification: Medium-term Dataset Expansion and Examples

## 1. Overview

Expand the project from a foundation repository into a practical social-data resource with broader dataset coverage, examples, and general user guides.

## 2. Scope

This track covers the next 6-12 months of work:

- Additional dataset packs from Stats NZ and ABS.
- At least one additional source agency beyond Stats NZ, ABS, and AIHW.
- General user guides under `docs/guides/`.
- Rust CLI examples over selected local Parquet data and source metadata. Python helper scripts are retained as maintainer evidence; R examples are retired from the validated capability set.
- Improved source metadata capture for codelists, units, caveats, licences, update cadence, and methodology.

## 3. Requirements

- New dataset packs must follow repository structure and documentation standards.
- Rust CLI example commands must be runnable from the repository checkout when local build/link resources are available.
- Guides must be useful for non-specialist users and must avoid unsupported statistical claims.
- Source metadata should distinguish official metadata from repository-derived notes.
- Live source blockers should be captured in session logs and TODOs.

## 4. Acceptance Criteria

- At least six additional dataset packs are proposed, and at least three are implemented.
- At least one non-Stats NZ/ABS/AIHW source agency is documented and integrated at pack level.
- At least three general guides exist in `docs/guides/`.
- At least two Rust CLI example commands cover committed or documented sample data, with Python helper scripts available as maintainer evidence.
- Codelist/unit/caveat metadata is captured for selected high-priority datasets.

## 5. Style and Standards

Follow `CONTRIBUTING.md`, `AGENTS.md`, and existing dataset-pack conventions.
