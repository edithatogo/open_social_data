# Specification: Scaffold Rust Core & DatasetProvider Abstraction

## 1. Overview
The goal of this track is to scaffold the core Rust workspace/crate that will host the high-performance data ingestion engine, and to define the `DatasetProvider` trait abstraction. This trait will allow modular addition of new API sources (e.g., Stats NZ, ABS) in a uniform, plugin-like manner.

## 2. Requirements & Architecture
- **Rust Initialization:** Initialize a Rust library crate in the repository root or standard subdirectory.
- **Trait Definition (`DatasetProvider`):**
  - Must use `async_trait` or native `async fn` in traits (if Rust edition is modern).
  - Define interfaces to:
    - Return provider metadata (name, supported datasets, versions).
    - Query/test connection health (`ping`).
    - Stream raw data records/pages (e.g., yielding deserialized values or Arrow record batches).
    - Transform/map custom formats to standardized layouts.
- **Common Types:**
  - Define schema types representing agency-agnostic data points.
  - Integrate with `polars` / `arrow` memory structures.
- **Mock Implementation:**
  - Implement a `MockProvider` conforming to `DatasetProvider` that returns static datasets.
  - Run comprehensive test suites validating the design.

## 3. Style and Standards
Follow styling specified in [code-styleguides.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/code-styleguides.md) and [workflow.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/workflow.md).
