---
title: Validation and Quality Gates
description: Rust build validation, CLI validation commands, Python maintenance checks, data quality assertions, and the release readiness checklist.
---

# Validation and Quality Gates

The Open Social Data project includes multiple layers of validation, from Rust build checks to data quality assertions and release readiness checks.

## Rust Build Validation

Run from the repository root with a target directory outside OneDrive:

```cmd
set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2
set CARGO_BUILD_JOBS=1
set CARGO_PROFILE_DEV_DEBUG=0

cargo check --all-targets
```

### Run tests

```cmd
set CARGO_PROFILE_TEST_DEBUG=0
cargo test
```

Tests cover:
- Provider metadata, listing, and fetch behaviour (ABS SDMX, Stats NZ OData)
- Conditional request headers and HTTP 304 handling
- Catalog sync with mock providers (partial success, missing provider errors)
- JSON and SQLite catalog load/save/search/upsert semantics
- Pipeline: schema validation, atomic Parquet writes, roundtrip reads
- Quality assertions: non-null, null-limit, unique, numeric range, allowed values
- Delta updater: append to Parquet, schema mismatch rejection
- Circuit breaker and retry policy behaviour
- CLI integration tests via `tests/cli_integration.rs`

### Clippy

```cmd
cargo clippy --all-targets -- -D warnings
```

### Format

```cmd
cargo fmt --check
```

## CLI Validation Commands

Three validation subcommands are built into the CLI:

### validate dataset-packs

Checks that every dataset directory under `datasets/` has the required structure:

```bash
open-social-data-cli validate dataset-packs
open-social-data-cli validate dataset-packs --root datasets
```

Checks:
- `README.md` present
- `SESSION_LOG.md` present
- `docs/data_dictionary*.md` present
- `docs/accessible_guide*.md` present
- `scripts/*.py` present

### validate source-metadata

Checks that dataset packs with `source_metadata.json` have valid metadata:

```bash
open-social-data-cli validate source-metadata
open-social-data-cli validate source-metadata --require-all
```

With `--require-all`, every dataset pack must include a `source_metadata.json` file.

### validate medium-term

Verifies that medium-term roadmap items (guides, examples, backlog, source metadata) are present:

```bash
open-social-data-cli validate medium-term
open-social-data-cli validate medium-term --run-examples
```

With `--run-examples`, also runs the built-in example commands to verify they produce output.

## Python Maintenance Checks

The `scripts/maintenance_check.py` script performs long-term maintenance checks:

```cmd
python scripts\maintenance_check.py
```

Checks performed:

| Check | Description |
|-------|-------------|
| Dataset pack integrity | Every dataset has README.md, SESSION_LOG.md, data dictionary, accessible guide, and fetch script |
| Placeholder detection | Scans markdown files for unresolved placeholders like `$(date)`, `YYYY-MM-DD` |
| URL presence | Verifies that every README has at least one source URL |
| Live URL check | Optional: HEAD requests to discovered URLs (`--live` flag, configurable `--timeout`) |
| Python AST syntax | Parses all `.py` files to verify they compile without syntax errors |

Optional live URL checking:

```cmd
python scripts\maintenance_check.py --live --timeout 15.0
```

## Data Quality Assertions

Quality assertions are defined in `src/quality.rs` and run over DataFrames retrieved from providers.

### Assertion types

| Assertion | Description |
|-----------|-------------|
| `NonNull { column }` | The column must contain no null values |
| `NullLimit { column, max_nulls }` | The column may have at most `max_nulls` null values |
| `Unique { column }` | All values in the column must be unique |
| `NumericRange { column, min, max }` | All values must fall within `[min, max]` |
| `AllowedValues { column, values }` | All values must be in the given set |

### Example

```rust
use open_social_data_core::{
    QualityAssertion, QualityReport, validate_quality,
};

let assertions = vec![
    QualityAssertion::non_null("MEASURE"),
    QualityAssertion::allowed_values("MEASURE", vec!["SALES", "PROFIT"]),
    QualityAssertion::numeric_range("OBS_VALUE", Some(0.0), None),
];

let report: QualityReport = validate_quality(&frame, &assertions)?;
if report.is_valid() {
    println!("All quality checks passed for {} rows", report.checked_rows);
} else {
    for issue in &report.issues {
        println!("  Issue: {} — {}", issue.column, issue.message);
    }
}

// Save atomically as JSON
report.save_atomic("quality_report.json")?;
```

### Quality gate in fetch flow

During a CLI fetch, the `provider_payload_assertions()` function runs a standard set of quality checks over the provider's payload DataFrame before the Parquet write. If assertions fail, the fetch still completes but the quality status is recorded as `Failed` in the catalog.

## Release Readiness Checklist

The full checklist is documented at `docs/technical/release_readiness_checklist.md`.

### Quick check (from repository root)

```cmd
set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2
set CARGO_BUILD_JOBS=1
set CARGO_PROFILE_DEV_DEBUG=0

:: Rust checks
cargo check --all-targets
set CARGO_PROFILE_TEST_DEBUG=0
cargo test
cargo clippy --all-targets -- -D warnings

:: CLI validation
cargo run --bin open-social-data-cli -- validate dataset-packs
cargo run --bin open-social-data-cli -- validate source-metadata
cargo run --bin open-social-data-cli -- validate medium-term --run-examples

:: Python syntax check
python scripts\maintenance_check.py

:: Git hygiene
git diff --check
```

### Expected results

- All Rust commands pass (check, test, clippy)
- All CLI validation commands report `OK`
- Python maintenance check exits with code 0
- No whitespace errors in `git diff --check`
- `CHANGELOG.md`, `TODO.md`, and Conductor track are updated before committing