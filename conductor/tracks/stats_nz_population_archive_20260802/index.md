# Track: Stats NZ population exact-edition archive

- [Specification](./spec.md)
- [Implementation plan](./plan.md)
- [Metadata](./metadata.json)
- GitHub issue: https://github.com/edithatogo/open_social_data/issues/42
- Hugging Face target: https://huggingface.co/datasets/edithatogo/riopa-public-data-archive

## Evidence

- Stats NZ publishes the selected workbook for *Subnational population estimates:
  At 30 June 2025 (provisional)* at a credential-free official HTTPS URL.
- The sampled workbook is 97,990 bytes with SHA-256
  `001e8a896cfb50f5ed17836dc815b235e3bcca55ee91c9869a2afaeb054b50a6`.
- The XLSX container passes CRC validation, contains seven worksheets and identifies
  the exact edition plus the published regional and territorial-authority tables.

## Limitations

- Hosted execution, immutable Hub revisions and receipt evidence remain pending.
- This track preserves the source edition; it does not transform or qualify estimates
  for a downstream model.
