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
