# Release Process

Releases should be small, source-backed, and reproducible.

## Versioning

Use semantic version tags for code and documentation releases:

```cmd
git tag v0.1.0
git push origin v0.1.0
```

Dataset source updates can be noted in release notes without implying the repository republishes the full official dataset.

## Release Checklist

Before tagging:

1. Run `python scripts\maintenance_check.py`.
2. Run `python scripts\validate_dataset_packs.py`.
3. Run the AIHW local validation/example commands when local Parquet extracts are included.
4. Run Rust checks from `docs/technical/release_readiness_checklist.md`.
5. Confirm `CHANGELOG.md` describes notable additions, changes, blockers, and validation exceptions.
6. Confirm source licences and caveats are documented for changed dataset packs.
7. Confirm no unintended generated caches, large files, credentials, or local catalog files are staged.

## Release Notes

Release notes should include:

* New or changed dataset packs.
* New validation or maintenance tooling.
* Source API changes and known live-access blockers.
* Breaking changes to scripts, CLI commands, or output schemas.
* Validation commands run and any environment-specific exceptions.
