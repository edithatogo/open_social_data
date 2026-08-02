---
license: other
language:
  - en
pretty_name: RIOPA Public Data Archive
tags:
  - new-zealand
  - geospatial
  - public-data
  - provenance
  - archival
---

# RIOPA Public Data Archive

This Hugging Face dataset stores immutable, source-specific public-data archive
packets acquired by the `edithatogo/open_social_data` GitHub Actions workflow.
Every packet keeps source metadata, raw paged responses, retrieval receipts,
completeness checks, SHA-256 checksums and explicit non-claims together.

## Source packets

`stats-nz-meshblock-2026` is the definitive Stats NZ meshblock boundary edition
as at 1 January 2026, obtained from the public ArcGIS Feature Service. The source
item identifies Creative Commons Attribution 4.0 International and requires
attribution to Stats NZ Tatauranga Aotearoa.

- Source item: <https://www.arcgis.com/home/item.html?id=4c023f2be0bf4993bdf7327f08b794bb>
- Feature service: <https://services2.arcgis.com/vKb0s8tBIA3bdocZ/ArcGIS/rest/services/Meshblock_2026/FeatureServer/0>
- Licence: <https://creativecommons.org/licenses/by/4.0/>
- Attribution: Stats NZ Tatauranga Aotearoa

`stats-nz-subnational-population-2025` preserves the exact provisional Stats NZ
workbook published for the 30 June 2025 edition. It contains estimated resident
population tables for regional councils, territorial authorities and Auckland
local-board areas, including reference dates from 2023 through 2025.

- Release: <https://www.stats.govt.nz/information-releases/subnational-population-estimates-at-30-june-2025/>
- Exact workbook: <https://www.stats.govt.nz/assets/Uploads/Subnational-population-estimates/Subnational-population-estimates-At-30-June-2025/Download-data/subnational-population-estimates-at-30-june-2025-provisional.xlsx>
- Source SHA-256: `001e8a896cfb50f5ed17836dc815b235e3bcca55ee91c9869a2afaeb054b50a6`
- Licence: <https://creativecommons.org/licenses/by/4.0/>
- Attribution: Stats NZ Tatauranga Aotearoa

## Integrity and use

Use a named Hugging Face commit revision and the packet's `manifest.json`.
Verify `checksums.sha256` before use. Mutable `latest.json` is discovery-only and
must not be used as an analytical or citation identity.

The Meshblock packet contains no population observations. The population packet
preserves a provisional publication without revising or finalising its estimates.
Neither packet makes completeness, legal-status, accessibility, facility, causal
or operational claims beyond its named source edition and captured bytes. The
source limitations and liability statements remain applicable.
