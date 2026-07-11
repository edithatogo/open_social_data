# Data.gov.au Provider

`open-social-data-cli` exposes the public Data.gov.au CKAN API as the
`data_gov_au` provider. The first curated dataset is
`freedom_of_information_statistics`, an aggregate OAIC dataset of Australian
government FOI requests, costs, and charges.

```powershell
cargo run --bin open-social-data-cli -- list --provider data_gov_au
cargo run --bin open-social-data-cli -- fetch data_gov_au freedom_of_information_statistics --output data/foi-statistics.parquet
```

The provider first reads the CKAN package metadata, selects the current CSV
resource whose name begins with `FOI requests, costs and charges`, then fetches
that resource. It preserves ETag and Last-Modified values through the common
conditional-fetch path.

This is aggregate government statistics. It is not a bulk export of individual
Right to Know requests or responses. Resource selection is metadata-driven so
that a newly published annual CSV can replace the previous one without changing
the provider identifier.
