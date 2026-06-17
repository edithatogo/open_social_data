# Provenance and Archival Policy

The repository must preserve trust by making source, derived, and generated artefacts easy to distinguish.

## Artefact Classes

* Official source data: data served by the source agency or downloaded from an official source URL.
* Derived local outputs: Parquet files, quality reports, summaries, or extracts produced by repository scripts.
* Generated documentation: README files, data dictionaries, accessible guides, release notes, and dashboards created or maintained in this repository.

Dataset documentation should identify which class each artefact belongs to and link to the official source wherever possible.

## Minimum Provenance Fields

Dataset packs should record:

* Source agency and official dataset name.
* Official source URL and API endpoint where applicable.
* Date accessed or processed.
* Script or command used to fetch/process data.
* Output path and format.
* Source licence and repository licence.
* Known caveats, suppression rules, provisional flags, and revisions.

## Archival Rules

* Keep old generated outputs only when they are small, useful as examples, and allowed by the source licence.
* Move superseded outputs into a dataset `data/archive/` folder only when they are intentionally retained.
* Prefer source fetch scripts and reproducible commands over committing large generated data.
* When a source API changes, preserve the old script until a replacement is validated or document why it was removed.
* Never archive credentials, cookies, access tokens, private contact details, or personal data not already released by the official source.
