# Contributing to Open Social Datasets

Thank you for helping make public social data easier to find, understand, and use. This project welcomes improvements to dataset documentation, fetch scripts, validation checks, examples, and the Rust data engine.

## Project Standards

Contributions should keep the repository useful for both technical and non-technical users:

* Link back to the official source for every dataset.
* Prefer reproducible scripts over manually edited derived data.
* Write clear plain-language explanations alongside technical metadata.
* Keep generated or downloaded data out of Git unless it is intentionally small and useful as an example.
* Do not commit API keys, private credentials, tokens, cookies, or personal data.

## Repository Layout

Use the existing structure:

* `datasets/<agency>/<dataset>/README.md` for dataset overview and access details.
* `datasets/<agency>/<dataset>/docs/data_dictionary.md` for column definitions.
* `datasets/<agency>/<dataset>/docs/accessible_guide.md` for a plain-language guide.
* `datasets/<agency>/<dataset>/scripts/` for dataset-specific scripts.
* `scripts/shared/` for reusable fetchers or API helpers.
* `docs/guides/` for general user guides.
* `src/` for Rust library and CLI code.

Templates live in `templates/`. Use them when adding new dataset packs.

## Adding or Updating a Dataset

For a new dataset, include:

1. A dataset directory under `datasets/<agency>/<dataset_identifier>/`.
2. A filled-out `README.md` with source links, access method, caveats, licence, and citation guidance.
3. A `docs/data_dictionary.md` describing expected or observed columns.
4. A `docs/accessible_guide.md` explaining the dataset in plain language.
5. A fetch or processing script when practical.
6. A `SESSION_LOG.md` or log entry summarising what was done and what remains.

When live access is blocked, document the blocker honestly and still add useful source links, expected schema notes, and reproducible commands.

## Script Guidelines

Python scripts should:

* Use paths relative to the script or repository, not machine-specific absolute paths.
* Create output directories as needed.
* Use `requests` timeouts and a clear `User-Agent`.
* Read secrets from environment variables only.
* Log key steps and failures.
* Fetch a small sample before running a full download when working with a new endpoint.

Shared helpers belong in `scripts/shared/`; dataset wrappers belong beside the dataset.

## Rust Guidelines

The Rust crate powers provider fetching, catalog sync, quality checks, and Parquet export.

Before submitting Rust changes, run:

```cmd
set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2
cargo fmt --check
cargo check --all-targets
cargo test
cargo clippy --all-targets -- -D warnings
```

On this Windows workspace, the `C:\tmp` target directory avoids OneDrive `target/` ACL issues.

## Documentation Guidelines

Good documentation should:

* Explain what the dataset measures and why it matters.
* Define units, time periods, geography, suppression/caveat flags, and revisions.
* Distinguish source data from repository-derived outputs.
* Use plain language in accessible guides.
* Include known limitations rather than hiding uncertainty.

## Data, Licensing, and Ethics

Only include data that can be redistributed under the source licence. If unsure, link to the source and provide fetch scripts instead of committing data.

Respect privacy protections and source caveats. Do not remove suppression flags or present small-cell data as exact when the source marks it as rounded, suppressed, provisional, or otherwise qualified.

## Pull Request Checklist

Before requesting review:

* The changed files are scoped to the task.
* Dataset source links and licences are included.
* Scripts run from the repository checkout without machine-specific paths.
* Tests or validation commands have been run where relevant.
* `TODO.md` and `CHANGELOG.md` are updated for completed work or meaningful changes.
* No secrets or large unintended data files are staged.

## Reporting Issues

When opening an issue, include:

* The dataset or command involved.
* The expected result.
* The actual result or error message.
* The operating system and relevant tool versions if the issue is technical.
* Links to source agency pages where applicable.

## Community Expectations

All contributors must follow the project `CODE_OF_CONDUCT.md`.
