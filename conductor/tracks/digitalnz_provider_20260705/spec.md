# Track Specification: DigitalNZ Provider and DNZ CLI Capability Alignment

## Overview

Add a first-class DigitalNZ integration path to `open_social_data` so curated DigitalNZ and New Zealand Gazette datasets can be managed through the existing provider, catalog, validation, and Parquet export workflow.

This track does not replace the standalone `dnz` CLI. `dnz` remains the low-level DigitalNZ API client, ad hoc search/facet tool, MCP server, and release package. `open_social_data` should consume the same capabilities for repeatable dataset workflows where records are normalized, catalogued, quality-checked, and exported as open social data assets.

## Functional Requirements

- Add a `digitalnz` provider to the provider registry.
- Support curated dataset IDs that map to DNZ-backed workflows, initially including:
  - `nz_gazette`
  - one general DigitalNZ search fixture dataset for provider validation
- Preserve DigitalNZ authentication through `DIGITALNZ_API_KEY`.
- Preserve persistent response caching through DNZ cache configuration where practical.
- Convert DNZ search/export records into `polars::DataFrame` outputs compatible with the existing `fetch` command.
- Expose provider metadata through `open-social-data-cli list --provider digitalnz`.
- Document equivalent command mappings between `dnz` and `open-social-data-cli`.
- Keep ad hoc DigitalNZ exploration in `dnz`; keep repeatable dataset fetching in `open_social_data`.

## Non-Functional Requirements

- Network-facing tests must be hermetic and use fixture or mock HTTP responses.
- Provider code should follow existing `DatasetProvider` patterns.
- The implementation should avoid duplicating DNZ API request logic when `dnz-core` can be reused cleanly.
- Secrets must not be written to catalog files, cache keys, manifests, or logs.
- Output must remain deterministic enough for catalog updates, tests, and future release automation.

## Acceptance Criteria

- `open-social-data-cli list --provider digitalnz` lists curated DigitalNZ-backed datasets.
- `open-social-data-cli fetch digitalnz nz_gazette --output <path>` produces a Parquet file and catalog entry.
- The DigitalNZ provider can be validated without live API access in CI.
- Documentation explains when to use `dnz` directly versus the `open_social_data` provider.
- GitHub issues exist for provider implementation, Gazette dataset normalization, and command/documentation alignment.

## Out of Scope

- Removing or merging the standalone `dnz` CLI.
- Implementing arbitrary `dnz search` pass-through flags inside `open-social-data-cli`.
- Publishing registry packages or release artifacts for either project.
- Live API harvests that require credentials during CI.
