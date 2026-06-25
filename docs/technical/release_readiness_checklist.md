# Release Readiness Checklist

This checklist records the local gates for the first Open Social Datasets release candidate. Live agency fetches are separate from local release readiness because public endpoints can be unavailable or rate-limited.

## Rust CLI Dataset Checks

Run from this Windows workspace with a target directory outside OneDrive:

```cmd
set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2
set CARGO_BUILD_JOBS=1
set CARGO_PROFILE_DEV_DEBUG=0

cargo run --bin open-social-data-cli -- validate dataset-packs
cargo run --bin open-social-data-cli -- validate source-metadata
cargo run --bin open-social-data-cli -- validate medium-term --run-examples
```

Expected result: dataset packs report `OK`, source metadata-backed packs report `OK`, and medium-term guides, backlog, examples, and metadata are present.

## Rust CLI Examples

Run from this Windows workspace with the same Cargo environment used for dataset checks:

```cmd
cargo run --bin open-social-data-cli -- examples myhospitals-summary --limit 5
cargo run --bin open-social-data-cli -- examples source-metadata-inventory
```

Expected result: example output is printed from local Parquet extracts and source metadata files without live network access.

## Python Helper Syntax Validation

Run:

```cmd
python scripts\maintenance_check.py
```

Expected result: all dataset pack checks pass and Python helper scripts parse through the no-bytecode AST syntax gate. Python remains useful for wrappers and maintainer checks, but validated user-facing examples are exposed through the Rust CLI commands above.

## AIHW Local Data Validation

Run:

```cmd
python datasets\aihw\myhospitals\scripts\validate_myhospitals_data.py
```

Expected result: local Parquet extracts are non-empty and required columns are present. Use the Rust CLI examples above for representative user-facing output.

## Rust Build Validation

Run from this Windows workspace with a target directory outside OneDrive:

```cmd
set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2
set CARGO_BUILD_JOBS=1
set CARGO_PROFILE_DEV_DEBUG=0
cargo check --all-targets
set CARGO_PROFILE_TEST_DEBUG=0
cargo test
cargo clippy --all-targets -- -D warnings
```

Expected result: all commands pass when sufficient temp/build disk is available. The explicit target directory avoids inherited ACL problems in the repository `target/` directory. Debug-info suppression keeps Windows builds smaller; if a test-profile link fails with `No space left on device`, free `%TEMP%` and `C:\tmp` build artifacts and rerun serially. The Rust CLI uses native Windows TLS to avoid the previous AWS-LC C build burden.

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
