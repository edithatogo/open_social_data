# Maintenance Operations

Open Social Datasets should be checked on a predictable cadence so dataset packs do not silently drift from their sources.

## Cadence

* Weekly: run static maintenance checks in CI.
* Monthly: run live URL checks from a normal network environment.
* Quarterly: review dataset freshness, source methodology pages, and unresolved dataset proposals.
* Before every release: run the release-readiness checklist and the maintenance checks together.

## Local Commands

Static checks:

```cmd
python scripts\maintenance_check.py
```

Optional live source-link checks:

```cmd
python scripts\maintenance_check.py --live --timeout 15
```

Static checks validate dataset pack completeness, required source URLs, placeholder markers, and Python script compilation. Live checks attempt HTTP checks for discovered documentation URLs, but they are not required for ordinary local validation because agency endpoints can be temporarily unavailable.

## Handling Failures

* Broken source links: open a broken source link issue and update the affected dataset docs when a replacement official source is found.
* Stale dataset notes: update the dataset `SESSION_LOG.md`, `README.md`, and data dictionary with the current source status.
* Script failures: reproduce locally with the smallest fetch possible and record whether the issue is local tooling, source schema change, authentication, rate limiting, or source outage.
* Placeholder markers: replace them with dated source-backed text or move the note into an explicit TODO.
