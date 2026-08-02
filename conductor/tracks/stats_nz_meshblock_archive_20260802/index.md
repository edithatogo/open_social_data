# Track: Stats NZ Meshblock 2026 hosted archive

- [Specification](./spec.md)
- [Implementation plan](./plan.md)
- [Metadata](./metadata.json)
- [Agent-panel qualification](./qualification-report.md)
- GitHub issue: https://github.com/edithatogo/open_social_data/issues/35
- Hugging Face target: https://huggingface.co/datasets/edithatogo/riopa-public-data-archive

## Evidence

- The source item is public, identifies CC BY 4.0 and attributes Stats NZ Tatauranga Aotearoa.
- The service reports 57,575 records and 16 source-documented null/non-digitised geometries.
- [GitHub Actions run 30750165664](https://github.com/edithatogo/open_social_data/actions/runs/30750165664)
  completed from workflow revision `728c4d651b02458f9712a6daff8c388ca3a6acad`.
- The public [Hugging Face packet](https://huggingface.co/datasets/edithatogo/riopa-public-data-archive/tree/3f2dc0a4d95a4fcb495551098d58fc5bce9c9202)
  is immutable at revision `3f2dc0a4d95a4fcb495551098d58fc5bce9c9202`; the receipt-bearing
  revision is `34c093646f884d7b57447231d6605e83739bb302`.
- Manifest SHA-256 is `1352a1693bba7dc6c090a56aedb89bd33c098985cde2bf3e74bd765990a19a5f`;
  payload-set SHA-256 is `706c6d39c497e643eb5989fc65d4824799d16ade197b4c808a4e2988722e9b14`.
- The manifest records 57,575/57,575 object IDs, 231 pages, 16 null geometries and a
  stable pre/post inventory. Independent hosted readback verified the manifest plus the
  first and final page digests.
- RIOPA downstream evidence was merged in
  [riopa-infrastructure PR 170](https://github.com/edithatogo/riopa-infrastructure/pull/170).
- The 2026-08-03 three-member agent panel independently reproduced packet integrity,
  provenance and lifecycle evidence and returned **PASS** with no Critical or High findings.

## Limitations

- This track archives one exact geography edition. It does not supply population or national
  accessibility evidence and does not complete the downstream RIOPA spatial-archive track.
- Hugging Face is the revision-addressed public archive target for this packet, not a claim of
  institutional preservation, source authority or analytical fitness.
