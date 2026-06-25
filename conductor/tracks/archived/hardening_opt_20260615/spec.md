# Specification: Ingestion Optimization and Pipeline Hardening

## 1. Overview
Enhance pipeline speed, robustness, and stability by integrating SIMD-accelerated deserializers, retry/backoff wrappers, panic isolation boundaries, and validation guards to handle malformed inputs securely.

## 2. Requirements & Robustness
- **Panic Isolation:** Wrap third-party endpoint parsers and custom mappings in panic-safety recovery boundaries (`catch_unwind`) to prevent a crash in one provider from taking down the entire daemon/CLI application.
- **Resource Exhaustion Defense:** Configure deserialization limits (maximum recursion depth, maximum single payload lengths) on the JSON/CSV parsing routines.
- **Connection Pools & Retries:** Configure `reqwest` connection reuse, timeouts, and exponential backoff.
- **SIMD Parsing:** Migrate parsers to `simd-json` or SIMD-backed deserialization to process bulk responses.

## 3. Style and Standards
Refer to [code-styleguides.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/code-styleguides.md) and [workflow.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/workflow.md).
