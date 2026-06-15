# Product Guidelines

## 1. Documentation & Writing Guidelines
- **Tone & Voice:** Informative, objective, and plain-language. Explain complex statistical data in clear terms without sacrificing technical accuracy.
- **Accessibility:** Avoid unnecessary jargon. When industry-specific terminology is required (e.g., specific classification codes like ANZSIC), provide clear inline explanations or links to definitions.
- **Structure:** All dataset guides must use standard templates, keeping layout consistent so users can easily compare Stats NZ and ABS datasets.

## 2. Rust Pipeline Architecture & Code Guidelines
To ensure a bleeding-edge, high-performance data engine, all Rust modules must adhere to the following principles:
- **Zero-Copy Deserialization:** Leverage libraries like `serde` and `arrow` to process raw files (CSV, JSON, XML) without unnecessary memory allocations.
- **Compile-Time Schema Validation:** Use Rust's type system to represent dataset schemas. Ensure invalid data structures fail to compile rather than panicking at runtime.
- **Asynchronous & Concurrent Processing:** Use `tokio` for non-blocking I/O when fetching from APIs, coupled with graceful rate limiting and backoff strategies.
- **SIMD Acceleration:** Utilize SIMD-accelerated parsing (e.g., `simd-json` or `polars` engine features) to achieve maximum processing throughput on multi-core systems.
- **Deterministic Data Pipelines:** Ensure all pipeline steps produce reproducible parquet outputs given the same raw input data.
