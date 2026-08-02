#!/usr/bin/env python3
"""Capture one approved public URL as a content-addressed archive packet."""

from __future__ import annotations

import argparse
import hashlib
import json
import urllib.request
import urllib.error
from datetime import UTC, datetime
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source-id", required=True)
    parser.add_argument("--url", required=True)
    parser.add_argument("--terms-url", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    request = urllib.request.Request(
        args.url,
        headers={"User-Agent": "open-social-data-archive/1.0", "Accept-Encoding": "identity"},
    )
    try:
        with urllib.request.urlopen(request, timeout=120) as response:
            body = response.read()
            headers = {key.lower(): value for key, value in response.headers.items()}
            receipt = {
                "url": response.geturl(),
                "status": response.status,
                "retrieved_at": datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
                "headers": {key: headers[key] for key in ("content-type", "etag", "last-modified", "content-length") if key in headers},
            }
            archive_status = "captured"
    except urllib.error.HTTPError as error:
        body = b""
        receipt = {
            "url": args.url,
            "status": error.code,
            "retrieved_at": datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
            "error": "http-error",
            "headers": {key.lower(): value for key, value in error.headers.items()},
        }
        archive_status = "unavailable"
    digest = hashlib.sha256(body).hexdigest()
    args.output.mkdir(parents=True, exist_ok=False)
    payload = args.output / "payload"
    if body:
        payload.write_bytes(body)
    manifest = {
        "schema_version": "1.0.0",
        "source_id": args.source_id,
        "terms_url": args.terms_url,
        "receipt": receipt,
        "payload": {"path": "payload", "bytes": len(body), "sha256": digest} if body else None,
        "archive_status": archive_status,
    }
    (args.output / "manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    (args.output / "checksums.sha256").write_text(f"{digest}  payload\n" if body else "", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
