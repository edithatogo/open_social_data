# Specification: Command-Line Interface (CLI) Application Development

## 1. Overview
Build a CLI application (`open-social-data-cli`) using Rust's `clap` parser, allowing users to fetch datasets, test API providers, list schemas, and view processed Parquet files.

## 2. Requirements
- **CLI Framework:** Integrate the `clap` crate (derive features for subcommands and arguments).
- **Subcommands:**
  - `list`: Show supported datasets and active providers.
  - `fetch`: Download data from a selected provider and output format (e.g. Parquet, CSV).
  - `status`: Check endpoint connectivity.
- **Reporting:** Format CLI outputs with sleek console indicators (using `indicatif` for progress bars and `colored` or similar for pretty-printing).

## 3. Style and Standards
Refer to [code-styleguides.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/code-styleguides.md) and [workflow.md](file:///C:/Users/60217257/OneDrive%20-%20Flinders/repos/legal-nz/open_social_data/conductor/workflow.md).
