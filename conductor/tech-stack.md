# Technology Stack

## Core Language & Runtime
- **Language:** Rust (Stable)
- **Async Runtime:** `tokio` (multi-threaded feature set)

## High-Performance Data Processing
- **DataFrames & Processing:** `polars` (fast, multi-threaded DataFrame library utilizing Arrow memory model)
- **Serialization & Parsing:** `serde` with `serde_json` and `csv` (zero-copy deserialization where applicable)
- **Parquet Storage:** `parquet` crate (for producing highly optimized columnar files)

## Networking & API Fetching
- **HTTP Client:** `reqwest` (asynchronous, pooled connections)
- **Rate-Limiting & Backoff:** `governor` or `tokio-retry` (resilient fetching from public agency endpoints)

## API Abstraction & Plugin Architecture
To easily onboard new statistical agencies and API sources, the project uses a trait-based abstraction model:
- **`DatasetProvider` Trait:** Defines common interfaces for checking status, fetching metadata, paging records, and streaming data.
- **Dynamic Registry:** Allows compiling and registering individual agency implementations (e.g., `AbsProvider`, `StatsNzProvider`) as modular pipelines without changing the core engine.
