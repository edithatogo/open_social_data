#!/usr/bin/env python3
"""Archive one exact-edition public Stats NZ population workbook."""

from __future__ import annotations

import argparse
import gzip
import hashlib
import json
import os
import time
import urllib.error
import urllib.parse
import urllib.request
import zipfile
from datetime import UTC, datetime
from pathlib import Path, PurePosixPath
from typing import Any

USER_AGENT = (
    "open-social-data-archive/1.0 (+https://github.com/edithatogo/open_social_data)"
)
AUTHORITY = "Stats NZ Tatauranga Aotearoa"
DATASET = "Subnational population estimates"
EDITION = "At 30 June 2025 (provisional)"
SOURCE_ID = "stats-nz-subnational-population-2025"
RELEASE_URL = (
    "https://www.stats.govt.nz/information-releases/"
    "subnational-population-estimates-at-30-june-2025/"
)
WORKBOOK_URL = (
    "https://www.stats.govt.nz/assets/Uploads/Subnational-population-estimates/"
    "Subnational-population-estimates-At-30-June-2025/Download-data/"
    "subnational-population-estimates-at-30-june-2025-provisional.xlsx"
)
EXPECTED_WORKBOOK_SHA256 = (
    "001e8a896cfb50f5ed17836dc815b235e3bcca55ee91c9869a2afaeb054b50a6"
)
EXPECTED_WORKBOOK_BYTES = 97_990
EXPECTED_WORKBOOK_MEDIA_TYPE = (
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
)
REQUIRED_MARKERS = (
    "Subnational population estimates: At 30 June 2025",
    "Estimated resident population, regional council areas, at 30 June 2023–2025",
    "Estimated resident population, territorial authority and Auckland local board areas, at 30 June 2023–2025",
)
RECEIPT_HEADERS = ("content-type", "etag", "last-modified", "content-length")


def utc_now() -> str:
    return datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def canonical_json(value: Any) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, sort_keys=True, separators=(",", ":")
    ).encode("utf-8")


def validate_stats_nz_url(value: str, expected: str) -> str:
    parsed = urllib.parse.urlparse(value)
    expected_parsed = urllib.parse.urlparse(expected)
    if (
        parsed.scheme != "https"
        or (parsed.hostname or "").lower() != "www.stats.govt.nz"
        or parsed.username
        or parsed.password
        or parsed.port
        or parsed.query
        or parsed.fragment
        or parsed.path != expected_parsed.path
    ):
        raise ValueError("source URL differs from the approved Stats NZ HTTPS URL")
    return value


def request_bytes(
    url: str, *, timeout: float, accept: str, attempts: int = 4
) -> tuple[bytes, dict[str, Any]]:
    request = urllib.request.Request(
        url,
        headers={
            "Accept": accept,
            "Accept-Encoding": "identity",
            "User-Agent": USER_AGENT,
        },
    )
    last_error: Exception | None = None
    for attempt in range(1, attempts + 1):
        try:
            with urllib.request.urlopen(request, timeout=timeout) as response:
                body = response.read()
                resolved = response.geturl()
                headers = {
                    name.lower(): value for name, value in response.headers.items()
                }
                return body, {
                    "url": resolved,
                    "method": request.get_method(),
                    "status": response.status,
                    "retrieved_at": utc_now(),
                    "headers": {
                        name: headers[name]
                        for name in RECEIPT_HEADERS
                        if name in headers
                    },
                }
        except (TimeoutError, urllib.error.URLError, urllib.error.HTTPError) as error:
            last_error = error
            if attempt == attempts:
                break
            time.sleep(min(2 ** (attempt - 1), 8))
    raise RuntimeError(f"request failed after {attempts} attempts: {url}: {last_error}")


def media_type(receipt: dict[str, Any]) -> str:
    return receipt.get("headers", {}).get("content-type", "").split(";", 1)[0].lower()


def validate_workbook(body: bytes) -> dict[str, Any]:
    import io

    try:
        with zipfile.ZipFile(io.BytesIO(body)) as workbook:
            if workbook.testzip() is not None:
                raise ValueError("workbook contains a corrupt ZIP member")
            names = workbook.namelist()
            for name in names:
                path = PurePosixPath(name)
                if path.is_absolute() or ".." in path.parts or "\\" in name:
                    raise ValueError("workbook contains an unsafe member path")
            required_members = {
                "[Content_Types].xml",
                "xl/workbook.xml",
                "xl/sharedStrings.xml",
            }
            if not required_members.issubset(names):
                raise ValueError("workbook lacks required XLSX members")
            shared_strings = workbook.read("xl/sharedStrings.xml").decode(
                "utf-8", "strict"
            )
            missing = [marker for marker in REQUIRED_MARKERS if marker not in shared_strings]
            if missing:
                raise ValueError(f"workbook edition markers are missing: {missing}")
            workbook_xml = workbook.read("xl/workbook.xml").decode("utf-8", "strict")
            worksheet_count = sum(
                name.startswith("xl/worksheets/sheet") and name.endswith(".xml")
                for name in names
            )
            if worksheet_count != 7:
                raise ValueError(
                    f"workbook sheet count changed: expected 7, observed {worksheet_count}"
                )
            return {
                "zip_members": len(names),
                "worksheets": worksheet_count,
                "workbook_xml_sha256": sha256_bytes(workbook_xml.encode("utf-8")),
                "required_edition_markers": list(REQUIRED_MARKERS),
            }
    except zipfile.BadZipFile as error:
        raise ValueError("source is not a valid XLSX workbook") from error


def github_execution() -> dict[str, str | None]:
    return {
        "repository": os.getenv("GITHUB_REPOSITORY"),
        "revision": os.getenv("GITHUB_SHA"),
        "run_id": os.getenv("GITHUB_RUN_ID"),
        "run_attempt": os.getenv("GITHUB_RUN_ATTEMPT"),
        "workflow": os.getenv("GITHUB_WORKFLOW"),
    }


def capture(args: argparse.Namespace) -> Path:
    release_url = validate_stats_nz_url(args.release_url, RELEASE_URL)
    workbook_url = validate_stats_nz_url(args.workbook_url, WORKBOOK_URL)
    started_at = utc_now()
    staging = args.output / ".staging"
    staging.mkdir(parents=True, exist_ok=False)

    release_body, release_receipt = request_bytes(
        release_url, timeout=args.timeout, accept="text/html"
    )
    validate_stats_nz_url(release_receipt["url"], RELEASE_URL)
    if media_type(release_receipt) != "text/html":
        raise ValueError("release landing page did not return HTML")
    if b"Subnational population estimates: At 30 June 2025" not in release_body:
        raise ValueError("release landing page lacks the approved edition title")

    workbook_body, workbook_receipt = request_bytes(
        workbook_url, timeout=args.timeout, accept=EXPECTED_WORKBOOK_MEDIA_TYPE
    )
    validate_stats_nz_url(workbook_receipt["url"], WORKBOOK_URL)
    if media_type(workbook_receipt) != EXPECTED_WORKBOOK_MEDIA_TYPE:
        raise ValueError("workbook returned an unexpected media type")
    observed_sha256 = sha256_bytes(workbook_body)
    if len(workbook_body) != args.expected_bytes:
        raise ValueError(
            f"workbook byte length changed: expected {args.expected_bytes}, "
            f"observed {len(workbook_body)}"
        )
    if observed_sha256 != args.expected_sha256:
        raise ValueError(
            "workbook SHA-256 changed from the approved exact edition: "
            f"expected {args.expected_sha256}, observed {observed_sha256}"
        )
    workbook_validation = validate_workbook(workbook_body)

    raw = staging / "raw"
    raw.mkdir(parents=True)
    workbook_path = raw / "subnational-population-estimates-at-30-june-2025-provisional.xlsx"
    workbook_path.write_bytes(workbook_body)
    release_gzip = gzip.compress(release_body, compresslevel=9, mtime=0)
    release_path = raw / "release-page.html.gz"
    release_path.write_bytes(release_gzip)

    files = [
        {
            "path": str(workbook_path.relative_to(staging)),
            "media_type": EXPECTED_WORKBOOK_MEDIA_TYPE,
            "bytes": len(workbook_body),
            "sha256": observed_sha256,
            "preserved_as_received": True,
        },
        {
            "path": str(release_path.relative_to(staging)),
            "media_type": "text/html",
            "content_encoding": "gzip",
            "bytes": len(release_gzip),
            "sha256": sha256_bytes(release_gzip),
            "uncompressed_bytes": len(release_body),
            "uncompressed_sha256": sha256_bytes(release_body),
        },
    ]
    payload_set_sha256 = sha256_bytes(
        canonical_json(
            [{"path": item["path"], "sha256": item["sha256"]} for item in files]
        )
    )
    capture_id = started_at.replace(":", "").replace("-", "")
    manifest = {
        "schema": "open-social-data.exact-file-archive.v1",
        "capture_id": capture_id,
        "started_at": started_at,
        "completed_at": utc_now(),
        "source": {
            "authority": AUTHORITY,
            "dataset": DATASET,
            "edition": EDITION,
            "release_url": release_url,
            "workbook_url": workbook_url,
            "expected_workbook_sha256": args.expected_sha256,
            "expected_workbook_bytes": args.expected_bytes,
            "license": "Creative Commons Attribution 4.0 International",
            "license_url": args.license_url,
            "attribution": AUTHORITY,
        },
        "scope": {
            "mode": "full-exact-edition",
            "workbook_validation": workbook_validation,
            "published_geographies": [
                "regional council areas",
                "territorial authority areas",
                "Auckland local board areas",
            ],
            "reference_dates": ["30 June 2023", "30 June 2024", "30 June 2025"],
        },
        "integrity": {
            "payload_set_sha256": payload_set_sha256,
            "source_identity_frozen": True,
            "files": files,
        },
        "retrieval_receipts": [
            {"artifact": files[1]["path"], **release_receipt},
            {"artifact": files[0]["path"], **workbook_receipt},
        ],
        "execution": github_execution(),
        "non_claims": [
            "This packet preserves the named provisional Stats NZ edition as published; it does not revise or finalise the estimates.",
            "Archival publication does not establish fitness for a particular analysis or downstream use.",
            "The mutable latest.json pointer is discovery metadata and is not an immutable citation identity.",
        ],
    }
    manifest_bytes = (
        json.dumps(manifest, indent=2, ensure_ascii=False, sort_keys=True).encode(
            "utf-8"
        )
        + b"\n"
    )
    (staging / "manifest.json").write_bytes(manifest_bytes)
    checksum_lines = [f"{item['sha256']}  {item['path']}" for item in files]
    checksum_lines.append(f"{sha256_bytes(manifest_bytes)}  manifest.json")
    (staging / "checksums.sha256").write_text(
        "\n".join(checksum_lines) + "\n", encoding="utf-8"
    )

    final = args.output / "snapshots" / args.source_id / capture_id
    final.parent.mkdir(parents=True, exist_ok=True)
    staging.rename(final)
    (args.output / "latest.json").write_text(
        json.dumps(
            {
                "source_id": args.source_id,
                "capture_id": capture_id,
                "manifest": str(final.relative_to(args.output) / "manifest.json"),
                "payload_set_sha256": payload_set_sha256,
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    return final


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument("--output", type=Path, required=True)
    result.add_argument("--source-id", default=SOURCE_ID)
    result.add_argument("--release-url", default=RELEASE_URL)
    result.add_argument("--workbook-url", default=WORKBOOK_URL)
    result.add_argument("--expected-sha256", default=EXPECTED_WORKBOOK_SHA256)
    result.add_argument("--expected-bytes", type=int, default=EXPECTED_WORKBOOK_BYTES)
    result.add_argument("--timeout", type=float, default=180.0)
    result.add_argument(
        "--license-url", default="https://creativecommons.org/licenses/by/4.0/"
    )
    return result


def main() -> None:
    args = parser().parse_args()
    if len(args.expected_sha256) != 64 or any(
        character not in "0123456789abcdef" for character in args.expected_sha256
    ):
        raise SystemExit("--expected-sha256 must be a lowercase SHA-256 digest")
    if args.expected_bytes < 1:
        raise SystemExit("--expected-bytes must be positive")
    final = capture(args)
    print(final)


if __name__ == "__main__":
    main()
