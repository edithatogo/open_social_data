# Validation Evidence

## 2026-06-18

Commands to run for this track:

```cmd
python scripts\validate_dataset_packs.py
python scripts\maintenance_check.py
python datasets\aihw\myhospitals\scripts\validate_myhospitals_data.py
python datasets\aihw\myhospitals\scripts\example_queries.py --limit 5
set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2
cargo check --all-targets
set CARGO_PROFILE_TEST_DEBUG=0
cargo test
cargo clippy --all-targets -- -D warnings
git diff --check
```

Results:

* `python scripts\validate_dataset_packs.py` - passed; all current ABS, Stats NZ, and AIHW packs reported `OK`.
* `python scripts\maintenance_check.py` - passed.
* `python datasets\aihw\myhospitals\scripts\validate_myhospitals_data.py` - passed for five local Parquet extracts:
  * `MYH-ADM`: 112,310 rows, 61 columns.
  * `MYH-CANCER`: 2,514 rows, 61 columns.
  * `MYH-ED-WAITS`: 41,489 rows, 61 columns.
  * `MYH-ES`: 2,000 rows, 61 columns.
  * `MYH-HH`: 2,000 rows, 61 columns.
* `python datasets\aihw\myhospitals\scripts\example_queries.py --limit 5` - passed and printed five current admission rows.
* `set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2&& cargo check --all-targets` - passed.
* `set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2&& cargo clippy --all-targets -- -D warnings` - passed.
* `set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2&& set CARGO_PROFILE_TEST_DEBUG=0&& cargo test` - passed after clearing stale build-cache disk pressure; 45 library unit tests, 5 CLI integration tests, and doc-tests completed with no failures.

Notes:

* `C:\tmp\open_social_data_target2` works for local Rust validation in this managed environment and avoids repository `target/` ACL inheritance problems.
* Full-debug `cargo test` builds can exhaust constrained Windows disks. Use `CARGO_PROFILE_TEST_DEBUG=0` for the release-readiness test gate.
