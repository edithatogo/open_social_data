# Track Specification: Stats NZ population exact-edition archive

## Overview

Acquire and preserve the exact public Stats NZ *Subnational population estimates:
At 30 June 2025 (provisional)* workbook through GitHub Actions and the existing
revision-addressed Hugging Face public-data archive.

## Authoritative inputs

- GitHub issue: https://github.com/edithatogo/open_social_data/issues/42
- Release: https://www.stats.govt.nz/information-releases/subnational-population-estimates-at-30-june-2025/
- Workbook: https://www.stats.govt.nz/assets/Uploads/Subnational-population-estimates/Subnational-population-estimates-At-30-June-2025/Download-data/subnational-population-estimates-at-30-june-2025-provisional.xlsx
- Approved source SHA-256: `001e8a896cfb50f5ed17836dc815b235e3bcca55ee91c9869a2afaeb054b50a6`
- Publication target: https://huggingface.co/datasets/edithatogo/riopa-public-data-archive

## Requirements

- Permit only the exact credential-free Stats NZ HTTPS release and workbook URLs.
- Require the frozen workbook byte length, SHA-256, media type, XLSX integrity,
  seven-sheet structure and edition-specific content markers.
- Preserve the workbook exactly as received with its landing page, HTTP receipts,
  manifest, checksums, execution identity and explicit non-claims.
- Publish using the existing protected GitHub environment and `HF_TOKEN`, then bind
  the immutable Hub packet revision to the GitHub run in a hosted receipt.

## Acceptance criteria

- Hermetic contract tests and repository checks pass locally and in GitHub Actions.
- The hosted packet contains the exact 97,990-byte workbook and validates against
  the approved SHA-256.
- A public Hugging Face revision and GitHub-bound receipt are independently readable.
- The Conductor evidence records the exact GitHub run, Hub revisions and manifest digest.

## Out of scope

- Treating this provisional edition as final or silently substituting later estimates.
- Transforming workbook cells, joining boundaries or making downstream analytical claims.
- Treating the mutable Hub default branch or `latest.json` as an immutable citation.
