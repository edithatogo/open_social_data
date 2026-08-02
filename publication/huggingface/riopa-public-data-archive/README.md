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

## Current source packet

`stats-nz-meshblock-2026` is the definitive Stats NZ meshblock boundary edition
as at 1 January 2026, obtained from the public ArcGIS Feature Service. The source
item identifies Creative Commons Attribution 4.0 International and requires
attribution to Stats NZ Tatauranga Aotearoa.

- Source item: <https://www.arcgis.com/home/item.html?id=4c023f2be0bf4993bdf7327f08b794bb>
- Feature service: <https://services2.arcgis.com/vKb0s8tBIA3bdocZ/ArcGIS/rest/services/Meshblock_2026/FeatureServer/0>
- Licence: <https://creativecommons.org/licenses/by/4.0/>
- Attribution: Stats NZ Tatauranga Aotearoa

## Integrity and use

Use a named Hugging Face commit revision and the packet's `manifest.json`.
Verify `checksums.sha256` before use. Mutable `latest.json` is discovery-only and
must not be used as an analytical or citation identity.

This archive does not contain population observations and makes no completeness,
legal-status, accessibility, facility, causal or operational claim beyond the
named source edition and captured bytes. The source's documented non-digitised
meshblocks and liability statement remain applicable.
