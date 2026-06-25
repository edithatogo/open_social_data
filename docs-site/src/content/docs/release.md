---
title: Release Process
description: How to version, tag, and release Open Social Data.
---

# Release Process

Releases should be small, source-backed, and reproducible.

## Versioning

Use semantic version tags for code and documentation releases:

```shell
git tag v0.1.0
git push origin v0.1.0
```

Dataset source updates can be noted in release notes without implying the repository republishes the full official dataset.

## Release Checklist

Before tagging, complete the following steps:

1. Run `python scripts/maintenance_check.py`
2. Run `cargo run --bin open-social-data-cli -- validate dataset-packs`
3. Run `cargo run --bin open-social-data-cli -- validate source-metadata`
4. Run `cargo run --bin open-social-data-cli -- validate medium-term --run-examples`
5. Run AIHW local data validator when local Parquet extracts are included
6. Run Rust checks from the release readiness checklist
7. Confirm `CHANGELOG.md` describes notable additions, changes, blockers, and validation exceptions
8. Confirm source licences and caveats are documented for changed dataset packs
9. Confirm no unintended generated caches, large files, credentials, or local catalog files are staged

On Windows, set `CARGO_TARGET_DIR`, `CARGO_BUILD_JOBS`, and `CARGO_PROFILE_DEV_DEBUG` before the Rust CLI gates.

## Release Notes

Release notes should include:

- New or changed dataset packs
- New validation or maintenance tooling
- Source API changes and known live-access blockers
- Breaking changes to scripts, CLI commands, or output schemas
- Validation commands run and any environment-specific exceptions

## Full Documentation

See [`docs/release_process.md`](https://github.com/edithatogo/open_social_data/blob/main/docs/release_process.md) in the repository for the complete release guide.