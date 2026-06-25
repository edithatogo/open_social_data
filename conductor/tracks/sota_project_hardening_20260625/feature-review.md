# Rust Dependency Feature Review

Review date: 2026-06-25
Track: sota_project_hardening_20260625 (Phase 3)

## reqwest (v0.13.4)

| Feature | Status | Rationale |
|---------|--------|-----------|
| `json` | Accepted (kept) | Required for ABS SDMX-JSON and Stats NZ OData JSON response parsing via `response.json::<T>()` |
| `native-tls-no-alpn` | Accepted (kept) | Required for TLS on the current Windows GNU toolchain |
| `gzip` | **Accepted (added)** | Enables transparent gzip decompression for compressed API responses. All modern agency APIs support gzip. Small binary overhead. |
| `stream` | Rejected | No streaming response consumers exist in the codebase. Can be added when chunked downloads are needed. |
| `brotli` / `zstd` | Rejected | `gzip` covers the common case. Brotli/zstd add dependency weight without confirmed agency support. |

## polars (v0.54)

| Feature | Status | Rationale |
|---------|--------|-----------|
| `parquet` | Accepted (kept) | Required for Parquet read/write via `ParquetReader`, `write_parquet_atomic`, `read_parquet` |
| `lazy` | **Accepted (added)** | Enables lazy query optimization and expression-based filtering. Default in most Polars usage. |
| `dtype-time` / `dtype-date` | **Accepted (added)** | Required for temporal dataset columns (TIME_PERIOD, date dimensions). Lightweight dtypes. |
| `csv` / `json` IO features | Rejected | Data arrives via API JSON (serde), not file-based CSV/JSON. Can be added for local data loading. |
| `sql` | Rejected | No SQL-over-DataFrame queries exist. Revisit if interactive catalog queries are needed. |

## rusqlite (v0.40.1)

| Feature | Status | Rationale |
|---------|--------|-----------|
| `bundled` | Accepted (kept) | Required for embedded SQLite (no system libsqlite3 dependency) |
| `backup` | **Accepted (added)** | Enables SQLite online backup API for catalog safety. Lightweight addition. |
| `limits` | **Accepted (added)** | Enables runtime resource limit configuration on SQLite connections. Good for defensive catalog operations. |
| `csv` | Rejected | No CSV import/export workflows exist in the catalog code. |
| `trace` | Rejected | Not useful without a tracing subscriber. Can be added for debugging sessions. |

## clap (v4.5.53)

| Feature | Status | Rationale |
|---------|--------|-----------|
| `derive` | Accepted (kept) | Required for `#[derive(Parser)]` and `#[derive(Subcommand)]` attribute macros |
| `unstable-doc` | Deferred to Phase 4 | Useful for generating markdown CLI reference. Will be reviewed during CLI polish. |

## Summary

- **4 features accepted (added)**: `reqwest/gzip`, `polars/lazy`, `polars/dtype-time + dtype-date`, `rusqlite/backup + limits`
- **2 features kept (unchanged)**: `clap/derive`, `reqwest/json + native-tls-no-alpn`, `polars/parquet`, `rusqlite/bundled`
- **7 features rejected**: `reqwest/stream`, `reqwest/brotli + zstd`, `polars/csv + json + sql`, `rusqlite/csv + trace`
- **1 feature deferred**: `clap/unstable-doc` (to Phase 4 CLI polish)