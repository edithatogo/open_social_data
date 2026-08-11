#!/usr/bin/env python3
"""Fail closed before a governed cultural-heritage archive run begins."""

from __future__ import annotations

import argparse
import os
from collections.abc import Mapping


SOURCES = {
    "new-zealand-gazette": {
        "secrets": ("HF_TOKEN", "DIGITALNZ_API_KEY", "IA_ACCESS_KEY", "IA_SECRET_KEY"),
        "url_variable": None,
        "hosts": (),
    },
    "auckland-museum-online-cenotaph": {
        "secrets": ("HF_TOKEN", "IA_ACCESS_KEY", "IA_SECRET_KEY"),
        "url_variable": "CENOTAPH_INVENTORY_URL",
        "hosts": ("api.aucklandmuseum.com",),
    },
    "papers-past-pre-1945": {
        "secrets": ("HF_TOKEN", "IA_ACCESS_KEY", "IA_SECRET_KEY"),
        "url_variable": "PAPERS_PAST_BULK_URL",
        "hosts": ("natlib.govt.nz", "paperspast.natlib.govt.nz"),
    },
}


def missing_requirements(source: str, environment: Mapping[str, str]) -> list[str]:
    definition = SOURCES[source]
    missing = [name for name in definition["secrets"] if not environment.get(name)]
    url_variable = definition["url_variable"]
    if url_variable and not environment.get(url_variable):
        missing.append(url_variable)
    return missing


def approved_url(source: str, value: str) -> bool:
    from urllib.parse import urlparse

    parsed = urlparse(value)
    return parsed.scheme == "https" and parsed.hostname in SOURCES[source]["hosts"]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", required=True, choices=sorted(SOURCES))
    args = parser.parse_args()
    missing = missing_requirements(args.source, os.environ)
    if missing:
        raise SystemExit(f"missing required GitHub secret or variable for {args.source}: {', '.join(missing)}")
    url_variable = SOURCES[args.source]["url_variable"]
    if url_variable and not approved_url(args.source, os.environ[url_variable]):
        raise SystemExit(f"{url_variable} must be an HTTPS URL on an approved first-party host")
    print(f"preflight passed for {args.source}; no capture or publication was performed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
