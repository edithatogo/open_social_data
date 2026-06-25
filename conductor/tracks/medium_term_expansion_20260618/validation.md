# Validation Evidence

## 2026-06-23

Commands for this track:

```cmd
cargo run --bin open-social-data-cli -- validate dataset-packs
cargo run --bin open-social-data-cli -- validate source-metadata
cargo run --bin open-social-data-cli -- validate medium-term --run-examples
cargo run --bin open-social-data-cli -- examples myhospitals-summary --limit 5
cargo run --bin open-social-data-cli -- examples source-metadata-inventory
python scripts\maintenance_check.py
git diff --check
```

Expected results:

* Dataset pack validation reports `OK` for ABS, Stats NZ, AIHW, and MoH packs.
* Source metadata validation reports `OK` for the Track 10 metadata-backed packs.
* Medium-term roadmap validation confirms candidate backlog, new source coverage, guides, examples, and metadata.
* Rust CLI examples run against committed local AIHW Parquet extracts and metadata files without live network access.

Results:

* Rust CLI validation commands were added after the initial Python-backed evidence pass so release validation can use the core CLI entry point.
* Earlier Python validator evidence passed for 11 dataset packs across ABS, Stats NZ, AIHW, and MoH.
* Earlier source metadata validation passed for the three Track 10 metadata-backed packs.
* Earlier medium-term roadmap validation passed.
* Earlier Python syntax validation passed.
* Earlier Python example helpers passed and printed local MyHospitals and source metadata summaries.
* R example capability was retired because `Rscript` is not available in this workspace.

## 2026-06-25 Rust CLI Follow-up

* `cargo fmt` - passed.
* `cargo check --all-targets --offline` with `CARGO_TARGET_DIR=C:\tmp\open_social_data_target_cli`, `CARGO_BUILD_JOBS=1`, and `CARGO_PROFILE_DEV_DEBUG=0` - passed. Re-run after Rust CLI validator parity fixes with `CARGO_TARGET_DIR=C:\tmp\open_social_data_target_check` also passed.
* `cargo test --test cli_integration --offline` and `cargo run --bin open-social-data-cli -- validate dataset-packs` were attempted serially after a failed parallel start, but full link/profile builds were blocked by local disk exhaustion while compiling AWS-LC C objects under `%TEMP%` and `C:\tmp`. No project source diagnostics were emitted before the disk-space failure.

Live-source notes:

* Stats NZ ADE and Ministry of Health explorer exports are endpoint-driven; exact export URLs should be captured during source refresh.
* ABS live DSD/data confirmation for QBIS and AWE remains blocked in this environment by DNS resolution for `api.abs.gov.au`.
## 2026-06-25 Native TLS Rust CLI Validation

Environment for Rust gates:

```cmd
set CARGO_TARGET_DIR=C:\tmp\open_social_data_target_native_tls_check
set CARGO_BUILD_JOBS=1
set CARGO_PROFILE_DEV_DEBUG=0
set CARGO_PROFILE_TEST_DEBUG=0
```

Results:

* `cargo tree -i aws-lc-sys --offline` - returned no matching package, confirming `aws-lc-sys` is no longer in the active dependency graph after switching `reqwest` to `default-features = false` with `native-tls-no-alpn`.
* `cargo tree -i native-tls --offline` - confirmed `open_social_data_core -> reqwest 0.13.4 -> native-tls` through `hyper-tls`/`tokio-native-tls`.
* `cargo check --all-targets --offline` - passed in 10m04s.
* `cargo run --bin open-social-data-cli -- validate dataset-packs` - passed and reported `OK` for 11 packs across ABS, Stats NZ, AIHW, and MoH.
* `cargo run --bin open-social-data-cli -- validate source-metadata` - passed and reported `OK` for `datasets\abs\average_weekly_earnings`, `datasets\moh\nz_health_survey_annual_update`, and `datasets\stats_nz\household_income_housing_costs`.
* `cargo run --bin open-social-data-cli -- validate medium-term --run-examples` - passed and reported `OK medium-term roadmap artefacts` after printing local MyHospitals and source metadata summaries.
* `cargo run --bin open-social-data-cli -- examples myhospitals-summary --limit 5` - passed against local AIHW Parquet extracts.
* `cargo run --bin open-social-data-cli -- examples source-metadata-inventory` - passed against local source metadata files.
* `cargo test --test cli_integration --offline` - passed: 7 tests.
* `cargo test --offline` - passed: 45 library tests, 0 binary tests, 7 CLI integration tests, and doctests clean with the existing ignored doctest.
* `cargo clippy --all-targets --offline -- -D warnings` - passed.
* `python scripts\maintenance_check.py` - passed, including dataset pack checks, README URL presence checks, placeholder checks, and no-bytecode AST syntax parsing for 24 Python files.
* `python scripts\validate_medium_term_roadmap.py --run-examples` - passed after the validator was corrected to invoke the Rust CLI examples instead of Python helper scripts. Python roadmap validator now invokes Rust CLI examples.
* `git diff --check` - passed.

The prior AWS-LC build/link blocker is resolved for this workspace by the native Windows TLS dependency path. Full local Rust validation now passes with a target directory outside OneDrive.