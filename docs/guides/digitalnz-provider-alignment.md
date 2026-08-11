# DigitalNZ Provider and DNZ CLI Alignment

Use `dnz` when you are exploring DigitalNZ interactively:

```bash
cargo run --bin dnz-cli -- search "kauri"
cargo run --bin dnz-cli -- gazette-export --output exports/gazette
```

Use `open-social-data-cli` when you want a curated dataset, a catalog entry,
and Parquet output that fits the rest of this repository's pipeline:

```bash
cargo run --bin open-social-data-cli -- list --provider digitalnz
cargo run --bin open-social-data-cli -- fetch digitalnz nz_gazette --output datasets/digitalnz/nz_gazette.parquet
cargo run --bin open-social-data-cli -- fetch digitalnz search_fixture --output datasets/digitalnz/search_fixture.parquet
cargo run --bin open-social-data-cli -- fetch auckland_museum cenotaph_metadata_sample --output datasets/auckland_museum/cenotaph_metadata/cenotaph-metadata.parquet
```

Environment:

- `DIGITALNZ_API_KEY` is required for live DigitalNZ requests.
- `DIGITALNZ_CACHE_PATH` or `DNZ_CACHE_PATH` can be used by `dnz` for
  persistent query caching.
- `open_social_data` keeps repeated fetch state in its local catalog and uses
  conditional requests when a dataset has prior ETag or Last-Modified values.

Recommended split:

- Use `dnz` for ad hoc search, facets, and Gazette export runs.
- Use `open_social_data` for curated DigitalNZ datasets that need consistent
  cataloging, validation, and Parquet export.
- Use `auckland_museum cenotaph_metadata_sample` as a bounded, metadata-only
  triangulation source for Cenotaph identifiers. It is not a licence to copy
  biographies, documents, images, or other payloads.

## Redundant archive policy

`open_social_data` is the canonical curated New Zealand Gazette ingestion
location. Do not create a second Gazette corpus in another repository. Keep
the authoritative Gazette URL, the DigitalNZ-derived normalized output, and an
independent preservation target (such as Internet Archive) in a single manifest
when a rights/privacy-reviewed capture is made.

For Papers Past, capture the pre-1-January-1945 **Crown** subset once item-level
provenance and cultural review are recorded. Do not treat the date alone as a
blanket licence for third-party newspaper, photograph, or Māori material.
