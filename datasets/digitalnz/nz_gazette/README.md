# New Zealand Gazette

This is the canonical `open_social_data` ingestion pack for New Zealand Gazette
notices. The Rust `digitalnz` provider creates normalized Parquet output; this
pack does not maintain a competing corpus elsewhere in the estate.

Authoritative source: <https://gazette.govt.nz/>.

Use `open-social-data-cli fetch digitalnz nz_gazette --output <path>` for a
curated provider capture. Full-preservation and Internet Archive redundancy
requirements are recorded in `config/acquisition/cultural-heritage-full-archive-plan.json`.
