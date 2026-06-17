# Release Readiness Checklist

This checklist records the local gates for the first Open Social Datasets release candidate. Live agency fetches are separate from local release readiness because public endpoints can be unavailable or rate-limited.

## Dataset Pack Completeness

Run:

```cmd
python scripts\validate_dataset_packs.py
```

Expected result: every current Stats NZ, ABS, and AIHW dataset pack reports `OK`.

## Python Script Compilation

Run:

```cmd
python -m compileall scripts datasets
```

Expected result: all shared and dataset-specific scripts compile.

## AIHW Local Data Validation

Run:

```cmd
python datasets\aihw\myhospitals\scripts\validate_myhospitals_data.py
python datasets\aihw\myhospitals\scripts\example_queries.py --limit 5
```

Expected result: local Parquet extracts are non-empty, required columns are present, and example queries print representative rows.

## Rust Validation

Run from this Windows workspace with a target directory outside OneDrive:

```cmd
set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2
cargo check --all-targets
cargo test
cargo clippy --all-targets -- -D warnings
```

Expected result: all commands pass. The explicit target directory avoids inherited ACL problems in the repository `target/` directory.

## ABS and Stats NZ Live Fetches

Dataset wrappers are present for the current ABS and Stats NZ packs. Live fetch validation is intentionally separate from release readiness:

* ABS wrappers call the Rust CLI and parse SDMX-JSON rows. If `api.abs.gov.au` is unavailable, record the endpoint error and retry later rather than treating the local release as failed.
* Stats NZ wrappers require an ADE JSON endpoint supplied with `--endpoint`; if an API key is needed, set it through the documented environment variable path rather than committing credentials.

## Git and Documentation

Run:

```cmd
git diff --check
```

Expected result: no whitespace errors. Update `CHANGELOG.md`, `TODO.md`, and the relevant Conductor track before committing.
