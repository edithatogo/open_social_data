# Contributor Workflow

This workflow keeps dataset changes reviewable and traceable.

## Dataset Ownership

Each dataset pack should have an owner or reviewer responsible for:

* Checking source links and methodology pages.
* Reviewing licence, caveat, suppression, and privacy notes.
* Confirming fetch scripts and validation commands still run.
* Updating `SESSION_LOG.md` when source access, schema, or known caveats change.

If no owner is assigned, maintainers should treat the pack as community-maintained and require a full source/validation review for substantive changes.

## Proposal to Merge

1. Open a dataset proposal issue using the template.
2. Confirm source authority, licence, access method, update cadence, and social value.
3. Create the dataset pack using repository templates.
4. Add or update scripts and validation notes.
5. Run `python scripts\maintenance_check.py` and relevant dataset checks.
6. Update `TODO.md`, `CHANGELOG.md`, and roadmap/Conductor status if the work closes a tracked item.
7. Request review from a maintainer or dataset owner.

## Review Criteria

Reviewers should check:

* Official source links are present and current.
* Licence terms allow the proposed repository use.
* Caveats, suppression, provisional status, revisions, and methodology limits are documented.
* Generated outputs are clearly distinguished from official source data.
* Scripts use repo-relative paths and do not require committed credentials.
* Validation commands are documented and pass or have a precise external blocker.

## Closing, Deferring, or Superseding Packs

* Close a dataset pack only when it is no longer maintained and a replacement source or archive note exists.
* Defer a pack when the source is valuable but blocked by access, licence, schema instability, or validation gaps.
* Supersede a pack when an official source changes endpoint, publication series, or methodology enough that a new pack is clearer than patching the old one.

Every closure, deferral, or supersession should be dated in `SESSION_LOG.md` and reflected in `TODO.md` if follow-up work remains.
