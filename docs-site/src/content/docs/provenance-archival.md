---
title: Provenance and Archival
description: Policy for preserving trust through provenance metadata and archival rules.
---

# Provenance and Archival Policy

The repository must preserve trust by making source, derived, and generated artefacts easy to distinguish.

## Artefact Classes

| Class | Description |
|-------|-------------|
| **Official source data** | Data served by the source agency or downloaded from an official source URL |
| **Derived local outputs** | Parquet files, quality reports, summaries, or extracts produced by repository scripts |
| **Generated documentation** | README files, data dictionaries, accessible guides, release notes, and dashboards |

Dataset documentation should identify which class each artefact belongs to and link to the official source wherever possible.

## Minimum Provenance Fields

Dataset packs should record:

| Field | Description |
|-------|-------------|
| Source agency | The authoritative agency (ABS, Stats NZ, AIHW, MoH) |
| Official dataset name | The name used by the source agency |
| Source URL | Official source URL and API endpoint where applicable |
| Date accessed | When the data was fetched or processed |
| Script used | The script or command used to fetch/process data |
| Output path | Where the output is stored (path and format) |
| Licence | Both source licence and repository licence |
| Caveats | Known caveats, suppression rules, provisional flags, and revisions |

## Archival Rules

- Keep old generated outputs only when they are small, useful as examples, and allowed by the source licence
- Move superseded outputs into a dataset `data/archive/` folder only when intentionally retained
- Prefer source fetch scripts and reproducible commands over committing large generated data
- When a source API changes, preserve the old script until a replacement is validated
- Never archive credentials, cookies, access tokens, private contact details, or personal data

## Full Documentation

See [`docs/provenance_archival_policy.md`](https://github.com/edithatogo/open_social_data/blob/main/docs/provenance_archival_policy.md) in the repository for the complete policy.