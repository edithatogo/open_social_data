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
- [GitHub Actions run 30751233863](https://github.com/edithatogo/open_social_data/actions/runs/30751233863)
  completed from main revision `9e96300e83b21b78f4116bc00fc141bf5f1efcad`.
- The public [Hugging Face packet](https://huggingface.co/datasets/edithatogo/riopa-public-data-archive/tree/4f94d300c0bea6b64972b4b67044990f7e591716/snapshots/stats-nz-subnational-population-2025/20260802T140246Z)
  is immutable at revision `4f94d300c0bea6b64972b4b67044990f7e591716`; the
  [GitHub-bound receipt](https://huggingface.co/datasets/edithatogo/riopa-public-data-archive/blob/bbe8136e4cac6052b7fb4ed5ac054822487d7aad/receipts/github/30751233863-1.json)
  is present at revision `bbe8136e4cac6052b7fb4ed5ac054822487d7aad`.
- Hosted readback verified manifest SHA-256
  `47540c8eb74fbc069b841308402961319aee57a0e85caad8b1de392595465617`,
  payload-set SHA-256 `9ea60cf23695db2f9c5c0b972fd6100776180916a93e50a4b63da24744b568c6`
  and the source workbook's approved SHA-256 and byte length.

## Limitations

- This track preserves the source edition; it does not transform or qualify estimates
  for a downstream model.
