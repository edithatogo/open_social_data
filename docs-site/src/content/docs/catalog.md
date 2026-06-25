---
title: Catalog System
description: How the JSON-backed and SQLite-backed catalogs work, including sync, list, search, and the CachedDataset metadata schema.
---

# Catalog System

The catalog tracks metadata about every dataset that has been discovered or fetched by the system. Two backends are supported: a JSON-backed catalog (default) and an SQLite-backed catalog (opt-in via `--sqlite`).

## JSON-Backed Catalog (default)

The default catalog is stored at `.open-social-data/catalog.json` as a local file. It is written atomically — the library writes to a `.tmp` file and renames it on success — to prevent corruption.

### Load and save

```rust
use open_social_data_core::LocalCatalog;

// Load from file (returns empty catalog if file doesn't exist)
let catalog = LocalCatalog::load(".open-social-data/catalog.json")?;

// Save atomically
catalog.save_atomic(".open-social-data/catalog.json")?;
```

### CLI commands

```bash
# List all datasets
open-social-data-cli catalog list

# List datasets from a specific provider
open-social-data-cli catalog list --provider abs

# Search by keyword
open-social-data-cli catalog search "population"
open-social-data-cli catalog search "cpi" --provider abs

# Sync provider metadata into the catalog
open-social-data-cli catalog sync
open-social-data-cli catalog sync --provider stats_nz
```

## SQLite-Backed Catalog

Use the `--sqlite` flag to use a SQLite database instead of JSON:

```bash
open-social-data-cli catalog list --sqlite my_catalog.sqlite
open-social-data-cli catalog search "cpi" --sqlite my_catalog.sqlite
open-social-data-cli catalog sync --sqlite my_catalog.sqlite
```

The SQLite catalog uses a `datasets` table with columns matching the `CachedDataset` schema. It is backed by the bundled `rusqlite` crate (no external SQLite installation required).

```rust
use open_social_data_core::SqliteCatalog;

// Open or create the SQLite database
let mut catalog = SqliteCatalog::open("my_catalog.sqlite")?;

// Upsert a dataset
catalog.upsert_metadata(dataset)?;

// Search
let results = catalog.search("cpi", Some("abs"))?;
```

## catalog sync

The `catalog sync` command contacts each registered provider's `list_datasets()` API endpoint, fetches the available dataset metadata, and upserts it into the local catalog.

Key behaviour:

- **Metadata-only:** Only metadata fields are written (name, version, source URL, catalog synced timestamp). Existing fetch output fields (output path, quality status, row count, ETag, `Last-Modified`) are preserved.
- **Partial success:** If some providers fail but others succeed, partial results are saved to the catalog and the errors are reported.
- **Provider filter:** Use `--provider <name>` to sync only a specific provider.

```bash
# Full sync of all registered providers
open-social-data-cli catalog sync

# Sync only the ABS provider
open-social-data-cli catalog sync --provider abs

# SQLite variant
open-social-data-cli catalog sync --sqlite my_catalog.sqlite
```

Output:
```
ok synced abs
ok synced stats_nz
synced 12 dataset metadata record(s)
```

If errors occurred:
```
ok synced abs
error stats_nz: API returned HTTP 503 for https://api.stats.govt.nz/opendata/v1
synced 5 dataset metadata record(s)
catalog sync completed with errors: stats_nz: API returned HTTP 503 ...
```

## catalog list

Lists every dataset in the catalog in a table-like format:

```
abs CPI rows=unknown quality=notrun synced=1742700000 etag=- output=-
abs LABOUR rows=unknown quality=notrun synced=1742700000 etag=- output=-
stats_nz Population rows=50000 quality=passed synced=1742700000 etag="xyz" output=pop.parquet
```

Each row shows:
- Provider name (green)
- Dataset ID
- `rows=` — row count (or `unknown`)
- `quality=` — quality status (`passed`, `failed`, `notrun`, or `unknown`)
- `synced=` — unix timestamp of the last catalog sync
- `etag=` — cached ETag value (or `-`)
- `output=` — output Parquet file path (or `-`)

## catalog search

Searches the catalog by case-insensitive substring matching against dataset IDs and names:

```bash
open-social-data-cli catalog search "price"
```

Returns the same row format as `catalog list`, filtered to matching datasets.

## CachedDataset Metadata Schema

The `CachedDataset` struct (`src/catalog.rs`) tracks the following fields for each dataset:

| Field | Type | Description |
|-------|------|-------------|
| `provider` | `String` | Provider name (e.g., `abs`) |
| `dataset_id` | `String` | Dataset identifier (e.g., `CPI`) |
| `name` | `Option<String>` | Human-readable name |
| `version` | `Option<String>` | Dataset version string |
| `catalog_synced_at` | `Option<String>` | Timestamp of last metadata sync |
| `last_fetched_at` | `Option<String>` | Timestamp of last successful data fetch |
| `last_not_modified_at` | `Option<String>` | Timestamp of last `304 Not Modified` response |
| `source_url` | `Option<String>` | Source API URL for the dataset |
| `etag` | `Option<String>` | Cached ETag header value |
| `last_modified` | `Option<String>` | Cached `Last-Modified` header value |
| `output_path` | `Option<PathBuf>` | Path to the saved Parquet file |
| `quality_status` | `Option<QualityStatus>` | `Passed`, `Failed`, or `NotRun` |
| `quality_report_path` | `Option<PathBuf>` | Path to the JSON quality report |
| `row_count` | `Option<usize>` | Number of rows in the fetched data |
| `not_modified_count` | `usize` | How many consecutive `304` responses received |

### QualityStatus enum

```rust
pub enum QualityStatus {
    Passed,
    Failed,
    NotRun,
}
```

### Fetch result merge behaviour

When a fetch updates the catalog (`upsert_fetch_result`), the following fields are preserved from any earlier metadata sync:

- `source_url`
- `catalog_synced_at`
- `name` and `version` (if the fetch entry has them as `None`)

This ensures that metadata from `catalog sync` is not lost when a fetch updates the catalog entry.