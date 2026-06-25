---
title: Getting Started
description: Install the Rust toolchain, build the CLI, and run your first data fetch from ABS or Stats NZ.
---

# Getting Started

Welcome to **Open Social Data**. This guide walks you through installing the tools, building the CLI, and fetching your first dataset.

## System Requirements

- **Rust toolchain** — Install via [rustup](https://rustup.rs/). The project uses the 2024 edition and requires Rust 1.84 or later.
- **Windows (current workspace)** — A GNU linker (`rustup default stable-x86_64-pc-windows-gnu`) is recommended to avoid Git `link.exe` conflicts. See `Cargo.toml` in the root for linker configuration notes.
- **Python 3.11+** — Required for dataset wrapper scripts and the maintenance check.
- **Node.js 20+ and npm** — Required if you want to build the Astro/Starlight documentation site locally.

## Clone and Build

```bash
git clone <repository-url>
cd open_social_data
```

### Rust CLI

Set a target directory outside OneDrive to avoid ACL issues on Windows:

```cmd
set CARGO_TARGET_DIR=C:\tmp\open_social_data_target2
set CARGO_BUILD_JOBS=1
set CARGO_PROFILE_DEV_DEBUG=0

cargo build --release
```

The release binary is placed at `target\release\open-social-data-cli.exe`.

### Python Dependencies

```bash
pip install -r requirements.txt
```

Required libraries include `pandas`, `pyarrow`, and `requests` for dataset wrapper scripts.

### Documentation Site (optional)

```bash
cd docs-site
npm install
npm run dev        # local preview at http://localhost:4321/open_social_data/
npm run build      # static site output
```

## Quickstart

### List available providers

```bash
cargo run --bin open-social-data-cli -- list
```

Output:
```
abs — Australian Bureau of Statistics
stats_nz — Stats NZ
```

### Sync the local catalog

```bash
cargo run --bin open-social-data-cli -- catalog sync
```

This contacts each provider's API, fetches the dataset listing, and stores the metadata in `.open-social-data/catalog.json`.

### List datasets in the catalog

```bash
cargo run --bin open-social-data-cli -- catalog list
```

Output example:
```
abs CPI rows=unknown quality=notrun synced=1742700000 etag=- output=-
stats_nz Population rows=unknown quality=notrun synced=1742700000 etag=- output=-
```

### Fetch a dataset

```bash
cargo run --bin open-social-data-cli -- fetch abs CPI --output cpi.parquet
```

Progress bars are shown during the fetch via the `indicatif` crate. On success, the dataset is saved as a Parquet file and the catalog is updated with quality status and row counts.

### Search the catalog

```bash
cargo run --bin open-social-data-cli -- catalog search "population"
```

## Running validation

```bash
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test
```

See the [Validation guide](./validation) for the full release readiness checklist.

---

## Next Steps

- Read the [CLI Reference](./cli-reference) for every command and flag.
- Learn how to [author a new provider](./providers).
- Understand the [dataset pack structure](./dataset-packs) for adding a new dataset.
- Explore the [catalog system](./catalog) for JSON and SQLite backends.
- See the [Roadmap and Track Status](./roadmap-status) for project progress.

For detailed guides on social statistics concepts, ethical use, and reading Parquet in Python, see the `docs/guides/` directory.