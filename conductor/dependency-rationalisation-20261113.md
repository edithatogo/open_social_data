# Rust dependency rationalisation — open_social_data (2026-11-13)

## Repository-specific decisions

- Verified: root already declares Anyhow, Clap, Polars 0.55, Reqwest 0.13, SQLite via rusqlite, Serde, Thiserror, Tokio full, and Criterion.
- Evaluate reducing Tokio full features and narrowing Reqwest TLS/features after exhaustive extraction of network, decompression and source-client needs; do not change provider licensing or resilience semantics.
- Retain Polars for tabular open statistics and rusqlite for database; do not add SQLx alongside working rusqlite without an explicit async-query requirement.
- Add Proptest on normalized ABS/Stats NZ observations, schema/dtype round-trips and deterministic parquet exports. Benchmark Polars lazy/eager plans with Criterion and realistic fixed-size datasets.
- Do not migrate provider format parsers to Regex, expose API secrets via Tracing, or add Rayon/Crossbeam without stable output ordering and measured throughput benefit.

## Evidence and implementation gates

- Status: **planned / scope registered**; no runtime substitution or CI/benchmark success is asserted by this file.
- Reconcile existing Conductor tracks before new work; preserve domain-specific APIs, schema versions, privacy and public CLI behaviour.
- Write failing contract tests before code changes, then run native targeted/full tests, fmt/clippy and relevant integration/supply-chain checks at exact SHA.
- Use Cargo feature/duplicate inventories plus security, licence and MSRV assessments, and fixed-fixture performance baselines before adoption.
- Record adopt / retain / reject / defer decisions and rollback; only close after verified evidence.
