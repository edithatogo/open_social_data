#!/usr/bin/env python3
"""Capture a public ArcGIS feature layer as an immutable, checksummed packet."""

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
from datetime import UTC, datetime
from pathlib import Path
from typing import Any

USER_AGENT = (
    "open-social-data-archive/1.0 (+https://github.com/edithatogo/open_social_data)"
)
ALLOWED_HOST_SUFFIXES = (".arcgis.com",)
RECEIPT_HEADERS = ("content-type", "etag", "last-modified", "content-length")


def utc_now() -> str:
    return datetime.now(UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def canonical_json(value: Any) -> bytes:
    return json.dumps(
        value, ensure_ascii=False, sort_keys=True, separators=(",", ":")
    ).encode("utf-8")


def sha256_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def validate_service_url(value: str) -> str:
    url = value.rstrip("/")
    parsed = urllib.parse.urlparse(url)
    hostname = (parsed.hostname or "").lower()
    if parsed.scheme != "https" or not any(
        hostname.endswith(suffix) for suffix in ALLOWED_HOST_SUFFIXES
    ):
        raise ValueError("service URL must use HTTPS on an arcgis.com host")
    if parsed.query or parsed.fragment or not parsed.path.endswith("/FeatureServer/0"):
        raise ValueError(
            "service URL must identify ArcGIS FeatureServer layer 0 without a query"
        )
    return url


def request_bytes(
    url: str,
    *,
    timeout: float,
    attempts: int = 4,
    form: dict[str, str] | None = None,
) -> tuple[bytes, dict[str, Any]]:
    request_body = urllib.parse.urlencode(form).encode("ascii") if form else None
    headers = {
        "Accept": "application/json",
        "Accept-Encoding": "identity",
        "User-Agent": USER_AGENT,
    }
    if request_body is not None:
        headers["Content-Type"] = "application/x-www-form-urlencoded"
    request = urllib.request.Request(
        url,
        data=request_body,
        headers=headers,
    )
    last_error: Exception | None = None
    for attempt in range(1, attempts + 1):
        try:
            with urllib.request.urlopen(request, timeout=timeout) as response:
                body = response.read()
                headers = {
                    name.lower(): value for name, value in response.headers.items()
                }
                receipt = {
                    "url": response.geturl(),
                    "method": request.get_method(),
                    "status": response.status,
                    "retrieved_at": utc_now(),
                    "headers": {
                        name: headers[name]
                        for name in RECEIPT_HEADERS
                        if name in headers
                    },
                }
                if request_body is not None:
                    receipt["request_body_sha256"] = sha256_bytes(request_body)
                return body, receipt
        except (TimeoutError, urllib.error.URLError, urllib.error.HTTPError) as error:
            last_error = error
            if attempt == attempts:
                break
            time.sleep(min(2 ** (attempt - 1), 8))
    raise RuntimeError(f"request failed after {attempts} attempts: {url}: {last_error}")


def arcgis_url(service_url: str, suffix: str, parameters: dict[str, str]) -> str:
    return f"{service_url}{suffix}?{urllib.parse.urlencode(parameters)}"


def parse_arcgis_json(body: bytes, label: str) -> dict[str, Any]:
    try:
        payload = json.loads(body)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise ValueError(f"{label} is not valid JSON") from error
    if not isinstance(payload, dict):
        raise TypeError(f"{label} must be a JSON object")
    if payload.get("error"):
        raise ValueError(f"{label} returned an ArcGIS error: {payload['error']}")
    return payload


def deterministic_gzip(value: bytes) -> bytes:
    return gzip.compress(value, compresslevel=9, mtime=0)


def write_capture(root: Path, relative: str, body: bytes) -> dict[str, Any]:
    compressed = deterministic_gzip(body)
    path = root / relative
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(compressed)
    return {
        "path": relative,
        "media_type": "application/json",
        "content_encoding": "gzip",
        "bytes": len(compressed),
        "sha256": sha256_bytes(compressed),
        "uncompressed_bytes": len(body),
        "uncompressed_sha256": sha256_bytes(body),
    }


def object_ids(payload: dict[str, Any], label: str) -> tuple[str, list[int]]:
    field = payload.get("objectIdFieldName")
    values = payload.get("objectIds")
    if not isinstance(field, str) or not isinstance(values, list):
        raise TypeError(f"{label} lacks objectIdFieldName/objectIds")
    ids = [int(value) for value in values]
    if len(ids) != len(set(ids)):
        raise ValueError(f"{label} contains duplicate object IDs")
    return field, sorted(ids)


def validate_page(
    payload: dict[str, Any],
    *,
    oid_field: str,
    expected_ids: list[int],
    final_page: bool,
) -> tuple[list[int], int]:
    features = payload.get("features")
    if not isinstance(features, list) or not features:
        raise ValueError("feature page is empty or malformed")
    observed: list[int] = []
    null_geometries = 0
    for feature in features:
        if not isinstance(feature, dict) or not isinstance(
            feature.get("attributes"), dict
        ):
            raise TypeError("feature page contains a malformed feature")
        observed.append(int(feature["attributes"][oid_field]))
        null_geometries += feature.get("geometry") is None
    if observed != expected_ids:
        raise ValueError(
            f"feature page object IDs differ from frozen inventory: expected "
            f"{expected_ids[0]}..{expected_ids[-1]}, observed {observed[0]}..{observed[-1]}"
        )
    if not final_page and len(observed) != len(expected_ids):
        raise ValueError("non-final feature page is incomplete")
    return observed, null_geometries


def github_execution() -> dict[str, str | None]:
    return {
        "repository": os.getenv("GITHUB_REPOSITORY"),
        "revision": os.getenv("GITHUB_SHA"),
        "run_id": os.getenv("GITHUB_RUN_ID"),
        "run_attempt": os.getenv("GITHUB_RUN_ATTEMPT"),
        "workflow": os.getenv("GITHUB_WORKFLOW"),
    }


def capture(args: argparse.Namespace) -> Path:
    service_url = validate_service_url(args.service_url)
    started_at = utc_now()
    staging = args.output / ".staging"
    staging.mkdir(parents=True, exist_ok=False)
    files: list[dict[str, Any]] = []
    receipts: list[dict[str, Any]] = []

    endpoints = {
        "item": f"https://www.arcgis.com/sharing/rest/content/items/{args.item_id}?f=json",
        "service": f"{service_url.rsplit('/', 1)[0]}?f=json",
        "layer": f"{service_url}?f=json",
    }
    metadata: dict[str, dict[str, Any]] = {}
    for name, url in endpoints.items():
        body, receipt = request_bytes(url, timeout=args.timeout)
        metadata[name] = parse_arcgis_json(body, name)
        files.append(write_capture(staging, f"raw/{name}.json.gz", body))
        receipts.append({"artifact": f"raw/{name}.json.gz", **receipt})

    item = metadata["item"]
    layer = metadata["layer"]
    if item.get("id") != args.item_id or item.get("access") != "public":
        raise ValueError("ArcGIS item identity is wrong or item is not public")
    if (
        args.expected_item_modified
        and int(item.get("modified", -1)) != args.expected_item_modified
    ):
        raise ValueError(
            "ArcGIS item modified timestamp changed from the approved source edition"
        )
    if layer.get("serviceItemId") != args.item_id:
        raise ValueError("layer serviceItemId does not match the approved ArcGIS item")

    ids_url = arcgis_url(
        service_url,
        "/query",
        {"where": "1=1", "returnIdsOnly": "true", "f": "json"},
    )
    ids_body, ids_receipt = request_bytes(ids_url, timeout=args.timeout)
    ids_payload = parse_arcgis_json(ids_body, "object ID inventory")
    oid_field, frozen_ids = object_ids(ids_payload, "object ID inventory")
    if args.expected_count and len(frozen_ids) != args.expected_count:
        raise ValueError(
            f"feature count changed: expected {args.expected_count}, observed {len(frozen_ids)}"
        )
    selected_ids = frozen_ids[: args.max_features or None]
    files.append(write_capture(staging, "raw/object-ids.json.gz", ids_body))
    receipts.append({"artifact": "raw/object-ids.json.gz", **ids_receipt})

    observed_ids: list[int] = []
    null_geometries = 0
    for page_number, offset in enumerate(
        range(0, len(selected_ids), args.page_size), start=1
    ):
        expected_ids = selected_ids[offset : offset + args.page_size]
        page_url = f"{service_url}/query"
        page_parameters = {
            "objectIds": ",".join(str(value) for value in expected_ids),
            "outFields": "*",
            "returnGeometry": "true",
            "returnM": "true",
            "returnZ": "true",
            "orderByFields": f"{oid_field} ASC",
            "f": "json",
        }
        body, receipt = request_bytes(
            page_url, timeout=args.timeout, form=page_parameters
        )
        payload = parse_arcgis_json(body, f"feature page {page_number}")
        page_ids, page_nulls = validate_page(
            payload,
            oid_field=oid_field,
            expected_ids=expected_ids,
            final_page=offset + args.page_size >= len(selected_ids),
        )
        relative = f"raw/features/page-{page_number:05d}.json.gz"
        files.append(write_capture(staging, relative, body))
        receipts.append({"artifact": relative, **receipt})
        observed_ids.extend(page_ids)
        null_geometries += page_nulls
        if args.delay:
            time.sleep(args.delay)

    post_body, post_receipt = request_bytes(ids_url, timeout=args.timeout)
    post_payload = parse_arcgis_json(post_body, "post-capture object ID inventory")
    post_field, post_ids = object_ids(post_payload, "post-capture object ID inventory")
    files.append(write_capture(staging, "raw/object-ids-post.json.gz", post_body))
    receipts.append({"artifact": "raw/object-ids-post.json.gz", **post_receipt})
    if post_field != oid_field or post_ids != frozen_ids:
        raise ValueError("source object ID inventory changed during capture")
    if observed_ids != selected_ids:
        raise ValueError(
            "captured feature sequence does not match the frozen inventory"
        )

    payload_set_sha256 = sha256_bytes(
        canonical_json(
            [{"path": item["path"], "sha256": item["sha256"]} for item in files]
        )
    )
    capture_id = started_at.replace(":", "").replace("-", "")
    manifest = {
        "schema": "open-social-data.arcgis-archive.v1",
        "capture_id": capture_id,
        "started_at": started_at,
        "completed_at": utc_now(),
        "source": {
            "authority": "Stats NZ Tatauranga Aotearoa",
            "dataset": "Meshblock 2026",
            "edition": "boundaries as at 1 January 2026",
            "arcgis_item_id": args.item_id,
            "arcgis_item_modified": item.get("modified"),
            "service_url": service_url,
            "service_item_id": layer.get("serviceItemId"),
            "spatial_reference": (layer.get("extent") or {}).get("spatialReference"),
            "copyright_text": layer.get("copyrightText"),
            "license": "CC BY 4.0",
            "license_url": args.license_url,
        },
        "scope": {
            "mode": "full" if not args.max_features else "bounded-sample",
            "available_features": len(frozen_ids),
            "captured_features": len(observed_ids),
            "null_geometries": null_geometries,
            "oid_field": oid_field,
            "first_oid": observed_ids[0],
            "last_oid": observed_ids[-1],
            "page_size": args.page_size,
            "pages": len(
                [item for item in files if item["path"].startswith("raw/features/")]
            ),
        },
        "integrity": {
            "payload_set_sha256": payload_set_sha256,
            "object_ids_sha256": sha256_bytes(canonical_json(frozen_ids)),
            "source_stable_during_capture": True,
            "files": files,
        },
        "retrieval_receipts": receipts,
        "execution": github_execution(),
        "non_claims": [
            "This packet is an archive of the named Stats NZ edition, not a population dataset.",
            "Publication does not imply that every geometry is digitised or error-free.",
            "No canonical, analytical, accessibility, facility or legal-status claims are made.",
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
    result.add_argument("--source-id", default="stats-nz-meshblock-2026")
    result.add_argument(
        "--service-url",
        default="https://services2.arcgis.com/vKb0s8tBIA3bdocZ/ArcGIS/rest/services/Meshblock_2026/FeatureServer/0",
    )
    result.add_argument("--item-id", default="4c023f2be0bf4993bdf7327f08b794bb")
    result.add_argument("--expected-item-modified", type=int, default=1765767642000)
    result.add_argument("--expected-count", type=int, default=57575)
    result.add_argument("--page-size", type=int, default=250)
    result.add_argument("--max-features", type=int, default=0)
    result.add_argument("--timeout", type=float, default=180.0)
    result.add_argument("--delay", type=float, default=0.05)
    result.add_argument(
        "--license-url", default="https://creativecommons.org/licenses/by/4.0/"
    )
    return result


def main() -> None:
    args = parser().parse_args()
    if args.page_size < 1 or args.page_size > 500:
        raise SystemExit("--page-size must be between 1 and 500")
    if args.max_features < 0:
        raise SystemExit("--max-features cannot be negative")
    final = capture(args)
    print(final)


if __name__ == "__main__":
    main()
