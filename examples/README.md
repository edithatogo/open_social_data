# Examples

This directory contains small, runnable examples for local Open Social Datasets outputs.

## Rust CLI

```cmd
cargo run --bin open-social-data-cli -- examples myhospitals-summary --limit 5
cargo run --bin open-social-data-cli -- examples source-metadata-inventory
```


The examples are intentionally small. They should run from the repository checkout and avoid live network access. `examples/python/` contains maintainer helper scripts, not validated user-facing examples. R examples are retired from the validated capability set.
