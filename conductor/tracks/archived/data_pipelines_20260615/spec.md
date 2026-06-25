# Specification: Arrow & Polars Data Transformation Pipeline and Parquet Export

## 1. Overview
Develop the data pipeline structure to compile raw structured payloads into optimized Arrow RecordBatches, process them via Polars DataFrames, and export them as compressed Parquet files safely.

## 2. Requirements & Robustness
- **DataFrame Integration:** Bind incoming API record streams to Arrow memory layouts.
- **Schema Alignment:** Setup strict runtime validation to ensure incoming row structures exactly align with targeted database/file schemas before starting the export.
- **Atomic File Writing:** Write output data to temporary files (e.g., `<file>.parquet.tmp`) and perform atomic replacement/rename upon successful completion. This guarantees that crash-loops or power losses never result in corrupted, half-written Parquet outputs.
- **Parquet Exporter:** Write optimized outputs using standard column-chunk compression (Snappy or Zstd) and partition layouts.

## 3. Style and Standards
Refer to [code-styleguides.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/code-styleguides.md) and [workflow.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/workflow.md).
