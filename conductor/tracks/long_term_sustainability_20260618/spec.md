# Specification: Long-term Sustainability and Advanced Access

## 1. Overview

Make Open Social Datasets maintainable as a long-lived public resource with reliable source checks, contributor workflow, releases, provenance, and optional advanced access tools.

## 2. Scope

This track covers 12+ month maturity work:

- Scheduled source-link, API, and documentation freshness checks.
- Contributor workflow for dataset proposals, review, ownership, and maintenance.
- Release and archival practices for scripts, metadata, and selected derived artefacts.
- Optional dashboards or interactive visualizations for high-value datasets.
- Cross-dataset analysis helpers for consistent geography, time, and indicator metadata.

## 3. Requirements

- Maintenance checks should be runnable locally and suitable for CI scheduling.
- Contribution workflow must be documented and practical for external contributors.
- Provenance metadata must distinguish official source data, derived local outputs, and generated documentation.
- Advanced tools must not make the core repository hard to validate or maintain.
- Privacy, suppression, caveats, and licensing must remain first-class concerns.

## 4. Acceptance Criteria

- A maintenance command or workflow checks source links, script health, and stale dataset notes.
- Dataset ownership and review workflow is documented.
- A release checklist and provenance/archival policy exists.
- At least one dashboard or advanced cross-dataset prototype exists, or a deliberate decision records why it is deferred.
- The long-term maintenance cadence is documented in the roadmap and contributor guidance.

## 5. Style and Standards

Follow `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `AGENTS.md`, and the repository's validation practices.
