# International youth-suicide mortality and HEOR

Source curator: **Tyler R. Black**. Upstream: https://pediatricsuicides.ca/.
Tracking: [open_social_data #66](https://github.com/edithatogo/open_social_data/issues/66).

This pack extends the existing source catalogue. It is **not a new repository,
validated mortality dataset, clinical risk tool or completed economic study**.

## Current state

The existing `config/acquisition/public-source-inventory.json` indexes six relevant
source families. This pack supplies source metadata, 22 research questions,
13 existing-asset references, 16 additional-data requirements and a four-layer
medallion contract. The landing page was inspected on 10 September 2026; export
bytes, licence notes and upstream headers were not acquired. Empirical rows: **0**.

Read [the HEOR register](docs/HEOR_QUESTIONS.md) for the full research/data map,
[the proposed dictionary](docs/data_dictionary.md), and [the integration and
rights plan](docs/INTEGRATION.md). `heor_register.json` is canonical for question
IDs, estimands, designs, asset references, gaps and analysis gates. Its prose
entries are design proposals, not findings.

## Use the existing architecture

GitHub owns catalogue definitions, code, contracts and study specifications.
Rights-permitted immutable source packets belong in the existing
`edithatogo/riopa-public-data-archive`; dataset identity/governance belongs in
`edithatogo/dataset-estate-registry`. Neither was modified in this slice.
Reconcile the complete current catalogue before submitting a new entry.

The ANZ policy atlas supplies policy provenance. `vop_poc_nz` is the candidate
study-specific economic consumer; `voiage` supplies qualified VOI methods;
`microcosting_healthworkforce` supplies costing logic requiring current inputs.
No code or data from those repositories is vendored here, and runtime integration
has not been qualified. Platinum is a rebuildable serving/semantic projection,
not an additional authoritative database or a fifth medallion layer.

## Offline checks

From the repository root:

```sh
python datasets/pediatricsuicides/international_youth_suicide/scripts/access.py plan
python datasets/pediatricsuicides/international_youth_suicide/scripts/access.py check
python -m unittest discover -s tests -p 'test_archive_youth_suicide.py' -v
```

The test filename is included by the existing CI maintenance job's
`test_archive*.py` pattern. Tests invoke the existing dataset-pack and metadata
validators directly, including this new source family. No new dependencies or
provider registration are required. This maintainer helper does **not** add a
`fetch pediatricsuicides` command to the Rust CLI.

## Acquisition only after rights review

Create a local receipt following `rights-receipt.template.json`, with exact
approved payload/terms URLs and a non-empty locally preserved terms document.
Its SHA-256 must match. Approval for acquisition and permission to redistribute
are separate. The template deliberately denies both and cannot authorise a fetch.

```sh
python datasets/pediatricsuicides/international_youth_suicide/scripts/access.py acquire \
  --rights-receipt /absolute/path/to/reviewed/rights.json \
  --output .open-social-data/youth-suicide/bronze/unique-reviewed-packet
```

The helper delegates to `scripts/archive_public_url.py`, verifies the returned
payload checksum, and preserves rights evidence. It refuses unsafe URLs, path
traversal and overwrites. A successful capture would still not qualify Silver,
prove the export is complete, or authorise publication. No automated fetch is
scheduled, and no synthetic fixture is placed in a public data layer.
