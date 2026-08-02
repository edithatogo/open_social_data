# Agent-panel qualification: Stats NZ Meshblock 2026 hosted archive

## Decision

**PASS — archive eligible within Track 15's stated Meshblock-only scope.**

The three-member agent panel found no Critical or High issues. Each member
independently verified the immutable packet, hosted execution and evidence chain.
The panel reviewed repository head `ebce8699c166c45d237f3078d5ee1c9e9931bc27`;
shared-pipeline remediation is recorded at `ee9bda0`.

Review date: 2026-08-03

## Panel

| Role | Verdict | Principal qualification |
| --- | --- | --- |
| Archive integrity and completeness | Pass | Downloaded the complete packet and verified every checksum, feature and inventory relationship. |
| Source, provenance and security | Pass | Verified the public Stats NZ identity, licence, attribution, receipts, credential boundary and non-claims. |
| Conductor lifecycle and operational evidence | Pass | Verified specification/plan acceptance, merged implementation history, hosted run, downstream handoff and archive continuity. |

## Independently reproduced evidence

- GitHub Actions run [30750165664](https://github.com/edithatogo/open_social_data/actions/runs/30750165664)
  succeeded at exact revision `728c4d651b02458f9712a6daff8c388ca3a6acad` with
  the full-inventory input (`max_features=0`). Every capture, checksum, publication,
  receipt and evidence-retention step passed.
- Implementation and remediation PRs
  [#38](https://github.com/edithatogo/open_social_data/pull/38),
  [#39](https://github.com/edithatogo/open_social_data/pull/39),
  [#40](https://github.com/edithatogo/open_social_data/pull/40) and
  [#41](https://github.com/edithatogo/open_social_data/pull/41) are merged with
  successful archive-contract, CI, documentation and security checks.
- Two panel members independently downloaded the complete immutable Hugging Face
  packet at revision `3f2dc0a4d95a4fcb495551098d58fc5bce9c9202`.
  All 237 checksum entries passed, covering 236 payload files plus the manifest.
- Manifest SHA-256 is
  `1352a1693bba7dc6c090a56aedb89bd33c098985cde2bf3e74bd765990a19a5f`;
  recomputed payload-set SHA-256 is
  `706c6d39c497e643eb5989fc65d4824799d16ade197b4c808a4e2988722e9b14`.
- The panel reconstructed 57,575 unique captured features in the exact frozen-ID
  sequence across 231 pages, with identical 57,575-ID pre/post inventories and 16
  null/non-digitised geometries. The object-ID digest matches the manifest.
- All 236 retrieval receipts map one-to-one to payload artifacts, use GET or POST,
  resolve to approved ArcGIS hosts and contain no token. Compressed and uncompressed
  sizes and digests validate.
- Live and captured item metadata agree on public item
  `4c023f2be0bf4993bdf7327f08b794bb`, modification timestamp `1765767642000`,
  Stats NZ Tatauranga Aotearoa attribution, CC BY 4.0, service identity and NZTM2000
  spatial reference.
- Receipt revision `34c093646f884d7b57447231d6605e83739bb302` binds the
  packet revision, manifest digest, GitHub repository/revision, run and attempt, and
  explicitly records that credentials were not retained.
- Downstream [RIOPA PR #170](https://github.com/edithatogo/riopa-infrastructure/pull/170)
  is merged at `5d2ca1a61d082a63480bdb41c53270db5f78f186` and records the
  exact packet revision and manifest digest.

## Validation

- `python3 -m unittest discover -s tests -p 'test_archive*.py'` — 14 passed after review remediation.
- `actionlint .github/workflows/archive-public-dataset.yml` — passed.
- `python3 -m py_compile scripts/archive_arcgis_feature_layer.py scripts/archive_stats_nz_population.py scripts/publish_hf_archive.py scripts/write_hf_archive_receipt.py` — passed.
- Complete hosted `sha256sum --check checksums.sha256` — 237/237 passed independently.
- Product guidelines — Pass: the archive is deterministic and preserves exact source/provenance evidence.
- Platform guides and `conductor/code_styleguides` — Not Applicable: none are selected or present for these paths.

## Review findings and remediation

1. **Medium, fixed:** final URLs after redirects were recorded but not revalidated.
   `ee9bda0` rejects non-HTTPS, non-ArcGIS, credential-bearing or fragmented final
   response URLs and adds regression tests.
2. **Medium, fixed:** `HF_TOKEN` was job-scoped. `ee9bda0` restricts it to the five
   publication-credential steps; acquisition and evidence-building steps cannot read it.
3. **Low, accepted:** Hugging Face frontmatter uses `license: other` because the target
   is a multi-source aggregate archive. The packet body and manifest state CC BY 4.0 and
   the required Stats NZ attribution, so no licence information is lost or misstated.
4. **Administrative, fixed by closeout:** the active registry link is replaced by the
   canonical archived `index.md` link and metadata/plan status are synchronized.

The two Medium findings do not invalidate the immutable 2026-08-02 packet: every
recorded final source URL was independently verified as an approved ArcGIS host, and
the packet, logs and receipt contain no credential material.

## Preserved scope and contingencies

- This is a qualification of the named Meshblock 2026 archival packet only.
- It does not qualify population observations, repaired or canonical geometries,
  accessibility, facilities, optimisation, legal status, analytical fitness or
  institutional preservation.
- Issue #35 originally combined geography and population scope. Track 15 closes only
  the geography packet; Track 16 separately owns the population archive.
- A future source identity, modification timestamp, feature count or inventory change
  requires a new exact-edition capture and qualification. It must not rewrite this packet.
