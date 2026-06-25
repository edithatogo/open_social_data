---
title: CLI Reference
description: Complete reference for the open-social-data-cli commands, subcommands, flags, exit codes, and output patterns.
---

# CLI Reference

The `open-social-data-cli` binary is the primary interface for listing providers, fetching datasets, managing the catalog, and running validation.

## Global Options

| Flag | Description |
|------|-------------|
| `--help` | Print help information |
| `--version` | Print version information |

## Commands

### `list`

List registered providers and their basic metadata.

```bash
open-social-data-cli list
open-social-data-cli list --provider abs
```

**Flags:**

| Flag | Description |
|------|-------------|
| `-p`, `--provider` | Filter to a specific provider name |

### `status`

Print a summary of the current project state, including the number of registered providers and cached datasets.

```bash
open-social-data-cli status
```

### `fetch`

Fetch a dataset from a provider and save it as a Parquet file.

```bash
open-social-data-cli fetch <provider> <dataset_id> --output <path>
```

**Arguments:**

| Argument | Description |
|----------|-------------|
| `provider` | Provider name (e.g., `abs`, `stats_nz`) |
| `dataset_id` | Dataset identifier from the provider's catalog |

**Flags:**

| Flag | Description |
|------|-------------|
| `-o`, `--output` | **Required.** Output Parquet file path |
| `--catalog` | Catalog file path (default: `.open-social-data/catalog.json`) |
| `--quality-report` | Optional path for the JSON quality report |

**Conditional fetches:** When the same dataset is fetched again, the CLI sends cached ETag and `Last-Modified` headers. If the server returns `304 Not Modified`, the output file is left unchanged and the catalog records the not-modified event.

**Progress bars:** The CLI shows a progress spinner via the `indicatif` crate during the fetch.

### `catalog`

Manage the local dataset catalog with three subcommands.

#### `catalog list`

List all datasets in the catalog.

```bash
open-social-data-cli catalog list
open-social-data-cli catalog list --provider stats_nz
open-social-data-cli catalog list --sqlite my_catalog.sqlite
```

Each row shows:
```
<provider> <dataset_id> rows=<count> quality=<status> synced=<timestamp> etag=<etag> output=<path>
```

#### `catalog search`

Search the catalog by keyword.

```bash
open-social-data-cli catalog search "population"
open-social-data-cli catalog search "cpi" --provider abs
open-social-data-cli catalog search "index" --sqlite my_catalog.sqlite
```

Searches match against dataset IDs and names (case-insensitive substring).

#### `catalog sync`

Sync provider dataset metadata into the local catalog without overwriting existing fetch output or quality fields.

```bash
open-social-data-cli catalog sync
open-social-data-cli catalog sync --provider abs
open-social-data-cli catalog sync --sqlite my_catalog.sqlite
```

Sync contacts each provider's `list_datasets()` API and upserts the returned metadata into the local catalog. Existing fetch results (output paths, quality status, row counts) are preserved.

**Output:**

```
ok synced abs
ok synced stats_nz
synced 12 dataset metadata record(s)
```

If some providers fail, partial results are saved and the errors are reported.

### `validate`

Run validation checks against dataset packs, source metadata, or medium-term roadmap items.

#### `validate dataset-packs`

Check that every dataset pack has the required directory structure and files.

```bash
open-social-data-cli validate dataset-packs
open-social-data-cli validate dataset-packs --root datasets
```

Checks for:
- `README.md` and `SESSION_LOG.md` in each dataset directory
- `docs/data_dictionary*.md` and `docs/accessible_guide*.md`
- At least one `*.py` script in `scripts/`

#### `validate source-metadata`

Check that dataset packs backed by a `source_metadata.json` have valid metadata.

```bash
open-social-data-cli validate source-metadata
open-social-data-cli validate source-metadata --require-all
```

With `--require-all`, every dataset pack must have a `source_metadata.json` file.

#### `validate medium-term`

Check medium-term roadmap items (guides, examples, backlog, metadata).

```bash
open-social-data-cli validate medium-term
open-social-data-cli validate medium-term --run-examples
```

With `--run-examples`, also runs the Rust CLI example commands to verify they produce output.

### `examples`

Run built-in example commands that demonstrate reading local Parquet data.

#### `examples myhospitals-summary`

```bash
open-social-data-cli examples myhospitals-summary --data-dir datasets/aihw/myhospitals/data --limit 5
```

Reads AIHW MyHospitals Parquet files from the given directory and prints summary rows.

#### `examples source-metadata-inventory`

```bash
open-social-data-cli examples source-metadata-inventory --root datasets
```

Walks the dataset tree and inventories all `source_metadata.json` files.

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | Any operation error (HTTP failure, parse error, IO error, validation failure) |

Errors use the library's typed error system (`CoreError`) and are printed to stderr:

```
Error: API returned HTTP 503 for https://api.abs.gov.au/data/CPI
```

## Backends

### JSON-backed catalog (default)

Datasets are stored in `.open-social-data/catalog.json`. The file is written atomically (write to `.tmp`, then rename) to prevent corruption.

### SQLite-backed catalog

Use the `--sqlite <path>` flag on `catalog list`, `catalog search`, or `catalog sync` to use a SQLite database instead. The schema stores the same `CachedDataset` fields in a `datasets` table. SQLite is useful when you want a queryable catalog file for tooling.

## Environment Notes

- Set `CARGO_TARGET_DIR` to a path outside OneDrive to avoid ACL issues with the Rust build cache.
- The CLI uses native Windows TLS (`native-tls-no-alpn`) to avoid the AWS-LC C build burden.