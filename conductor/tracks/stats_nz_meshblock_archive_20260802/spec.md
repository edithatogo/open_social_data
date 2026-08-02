# Track Specification: Stats NZ Meshblock 2026 hosted archive

## Overview

Acquire the exact public Stats NZ Meshblock 2026 ArcGIS layer through a bounded,
fail-closed GitHub Actions workflow and preserve the verified packet in
`edithatogo/riopa-public-data-archive` on Hugging Face.

## Authoritative inputs

- GitHub issue: https://github.com/edithatogo/open_social_data/issues/35
- ArcGIS item: https://www.arcgis.com/home/item.html?id=4c023f2be0bf4993bdf7327f08b794bb
- Layer: https://services2.arcgis.com/vKb0s8tBIA3bdocZ/ArcGIS/rest/services/Meshblock_2026/FeatureServer/0
- Licence: https://creativecommons.org/licenses/by/4.0/
- Downstream plan: https://github.com/edithatogo/riopa-infrastructure/blob/main/docs/public-dataset-archive-incorporation-plan-20260802.md

## Requirements

- Freeze the item identity, item modification timestamp and complete object-ID inventory.
- Preserve service, layer, item, inventory and feature-page response bytes separately.
- Reject wrong identity, private access, count drift, duplicate/missing IDs, malformed pages,
  ArcGIS errors and source changes during capture.
- Record source terms, HTTP receipts, raw/compressed SHA-256 digests, null geometry count,
  GitHub revision/run and explicit non-claims without retaining credentials.
- Publish only through the protected `HF_TOKEN` GitHub secret and bind the resulting Hub
  commit revision to a second hosted receipt.

## Acceptance criteria

- Hermetic archive contract tests pass locally and in GitHub Actions.
- The full 57,575-ID inventory is captured once with no missing or duplicate feature IDs.
- The packet and hosted receipt exist at immutable Hugging Face commit revisions.
- Checksums validate after downloading the named Hub revision.
- RIOPA records the exact manifest digest and Hub revision before incorporation.

## Out of scope

- Population observations, demographic denominators or census measures.
- Canonical geometry conversion, simplification, repair or analytical incorporation.
- Treating the mutable Hub default branch or `latest.json` as an immutable identity.
- Claiming that the 16 source-documented non-digitised meshblocks contain geometry.
