#!/usr/bin/env python3
"""Write a redacted receipt binding an archive packet to a Hugging Face revision."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from datetime import UTC, datetime
from pathlib import Path


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dataset", required=True)
    parser.add_argument("--revision", required=True)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    try:
        manifest_sha256 = sha256_file(args.manifest)
    except OSError as error:
        raise SystemExit(
            f"unable to read archive manifest {args.manifest}: {error}"
        ) from error
    payload = {
        "schema": "open-social-data.hugging-face-receipt.v1",
        "recorded_at": datetime.now(UTC)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z"),
        "dataset": args.dataset,
        "packet_revision": args.revision,
        "manifest_path": str(args.manifest),
        "manifest_sha256": manifest_sha256,
        "github": {
            "repository": os.getenv("GITHUB_REPOSITORY"),
            "revision": os.getenv("GITHUB_SHA"),
            "run_id": os.getenv("GITHUB_RUN_ID"),
            "run_attempt": os.getenv("GITHUB_RUN_ATTEMPT"),
        },
        "credentials_recorded": False,
    }
    try:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(
            json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
    except OSError as error:
        raise SystemExit(
            f"unable to write archive receipt {args.output}: {error}"
        ) from error


if __name__ == "__main__":
    main()
