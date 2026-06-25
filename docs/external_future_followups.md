# External and Future Follow-ups

These items are intentionally outside the completed Conductor roadmap. They should not be treated as blockers for Tracks 1-11.

## External Source Checks

- Re-check ABS live DSD/data codelists for QBIS and Average Weekly Earnings when `api.abs.gov.au` is reachable from the local environment.
- Refresh source metadata when agency explorers or generated download URLs change.

## Future Dataset Expansion

- Additional AIHW MyHospitals measure categories can be added as new dataset packs if they become a priority.
- Remaining medium-term candidate datasets in `conductor/tracks/medium_term_expansion_20260618/dataset_candidates.md` should be implemented only after source metadata, licence, and refresh paths are confirmed.

## Future Hardening

- SIMD-enabled deserialization, explicit maximum-depth/payload-limit enforcement, and malformed-payload memory/CPU benchmark suites are future performance/security enhancements. The current roadmap hardening scope is complete with retry, circuit breaker, panic isolation, conditional fetches, and local validation.

## Release-Time Actions

- `cargo publish --dry-run`, registry credentials, and any actual publication/upload steps are release-time actions that require explicit maintainer approval and should not block the local roadmap completion state.