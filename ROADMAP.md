# Project Roadmap

This roadmap converts the Open Social Datasets project from a completed foundation into a staged delivery plan. It is organized by time horizon and backed by Conductor tracks so each horizon can be implemented, reviewed, and closed.

## Current Position

Phase 1 foundation work is complete:

* Repository structure, templates, contribution guidance, and code of conduct are in place.
* Rust data engine core, providers, Parquet pipeline, quality checks, CLI, local JSON/SQLite catalog, and hardening tracks are complete.
* Initial dataset packs now exist for Stats NZ, ABS, and AIHW MyHospitals.
* Validation has been proven locally with Windows GNU Rust tooling and Python checks for the AIHW Parquet extracts.

The short-, medium-, and long-term tracks are implemented locally. Remaining source-refresh and optional expansion items are tracked separately as external/future follow-ups.

---

## Short-Term Roadmap: Stabilize and Complete the Current Foundation

**Conductor Track:** [Track 9: Short-term completion and source validation](./conductor/tracks/short_term_completion_20260618/)

**Goal:** Turn the current repository into a polished, reproducible first release candidate.

### Outcomes

* Refresh stale roadmap/TODO references and align all status files.
* Finish or explicitly retire the older ABS QBIS on-hold tasks.
* Validate dataset-pack scripts without live credentials where possible.
* Add smoke tests for dataset wrapper scripts and documented commands.
* Build a release-readiness checklist covering Rust, Python, documentation, and data artefacts.

### Done When

* `TODO.md`, `ROADMAP.md`, `CHANGELOG.md`, and Conductor tracks agree on what is complete and what is deferred.
* Every dataset pack has README, data dictionary, accessible guide, access script, and session log or documented reason for omission.
* Local validation commands are documented and pass without requiring live agency credentials.
* Known external blockers, such as ABS DNS/API endpoint access, are recorded as explicit follow-up items rather than loose TODOs.

---

## Medium-Term Roadmap: Expand Coverage and Examples

**Conductor Track:** [Track 10: Medium-term dataset expansion and examples](./conductor/tracks/medium_term_expansion_20260618/)

**Goal:** Broaden dataset coverage and give users clear examples for analysis and reuse.

### Outcomes

* Add more Stats NZ and ABS dataset packs using the established structure.
* Add at least one additional source agency beyond Stats NZ, ABS, and AIHW.
* Create general user guides for social statistics concepts, visual interpretation, and ethical use.
* Add Rust CLI examples for selected popular datasets and source metadata. Python helper scripts remain maintainer evidence; R examples are retired from the validated capability set.
* Improve source metadata capture, including codelists, units, update cadence, licences, caveats, and official methodology links.

### Done When

* [Done] The repository has enough dataset breadth to support common population, labour, cost-of-living, health, and macroeconomic workflows.
* [Done] Rust CLI example commands can read local Parquet data and produce simple summary tables.
* [Done] Guides explain how to interpret common social statistics without assuming advanced statistical training.
* [Done] New dataset packs follow the same documentation and validation standard as the initial packs.

Implementation status: Track 10 dataset, guide, example, metadata, Rust CLI validation, review, and commit work is complete.

---

## Long-Term Roadmap: Sustainability, Community, and Advanced Tools

**Conductor Track:** [Track 11: Long-term sustainability and advanced access](./conductor/tracks/long_term_sustainability_20260618/)

**Goal:** Make the project maintainable as a public resource rather than a one-off repository.

### Outcomes

* Establish recurring source-link and dataset freshness checks.
* Define review, ownership, and contribution workflows for community maintainers.
* Add optional dashboards or interactive visualizations for selected high-value datasets.
* Explore cross-dataset analysis tooling, including consistent geography, time, and indicator metadata.
* Prepare release, archival, and provenance practices for long-term trust.

### Done When

* [Done] Maintenance checks can identify broken links, stale scripts, and outdated documentation.
* [Done] Contributors have a clear path from issue to dataset proposal to reviewed merge.
* [Done] At least one advanced access or visualization prototype exists and is documented.
* [Done] The repository has a documented long-term maintenance cadence and release process.

---

## Roadmap Principles

* Prefer source-backed documentation over unsupported summaries.
* Keep machine-readable metadata and human-readable guides aligned.
* Treat caveats, suppression, provisional status, and licences as first-class data.
* Make small, reviewed tracks rather than broad unbounded work items.
* Document blockers honestly and keep local validation separate from live agency availability.

---

*This roadmap is a living document and should be updated whenever a Conductor track is completed, deferred, or superseded.*
