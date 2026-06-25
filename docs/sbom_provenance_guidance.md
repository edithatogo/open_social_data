# SBOM and Provenance Guidance

## Software Bill of Materials (SBOM)

Open Social Data uses the following mechanisms to track dependencies and generate SBOMs:

### Cargo Dependencies

Rust dependencies are tracked in `Cargo.lock` (generated automatically by Cargo). To generate an SPDX or CycloneDX SBOM:

```shell
# Install cargo-cyclonedx
cargo install cargo-cyclonedx --locked
# Generate CycloneDX SBOM
cargo cyclonedx
```

### Python Dependencies

Python dependencies are listed in `requirements.txt`. To generate a pip SBOM:

```shell
pip install cyclonedx-bom
cyclonedx-py requirements.txt --output-format json -o sbom.python.json
```

### npm Dependencies

npm dependencies are tracked in `docs-site/package-lock.json`. To generate an npm SBOM:

```shell
npm sbom --prefix docs-site
```

## Provenance

All dataset packs must record provenance metadata in their `source_metadata.json`:

| Field | Required | Description |
|-------|----------|-------------|
| agency | Yes | Source agency name and acronym |
| dataset_id | Yes | Identifier used by the source agency |
| title | Yes | Official dataset title |
| source_url | Yes | Official landing page or API endpoint |
| access_notes | Yes | Authentication, rate limits, or endpoint quirks |
| licence | Yes | SPDX identifier or free-text licence reference |
| methodology | Yes | URL or description of methodology documentation |
| caveats | Yes | Known caveats, suppression rules, revisions |

See `docs/provenance_archival_policy.md` for the full provenance and archival policy.

## CI Integration

The CI workflow runs the following provenance-adjacent checks:

- `cargo deny check` — license policy and dependency source validation
- `cargo audit` — security advisory scanning of Cargo dependencies

For release builds, run the full SBOM generation commands above before tagging.