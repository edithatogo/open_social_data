#!/usr/bin/env python3
"""Inspect HEOR contracts or acquire a rights-approved source using the shared archiver.

No network access occurs without the explicit acquire subcommand and a verified
local rights receipt. This helper is not a new Rust provider or a licence oracle.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
from datetime import date
from pathlib import Path
from urllib.parse import urlsplit

PACK = Path(__file__).resolve().parents[1]
ROOT = Path(__file__).resolve().parents[4]
SOURCE_ID = "pediatricsuicides-international"
ALLOWED_HOSTS = {"pediatricsuicides.ca", "www.pediatricsuicides.ca", "zenodo.org"}


def read_object(path: Path) -> dict:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise ValueError(f"Expected a JSON object: {path.name}")
    return value


def checked_url(value: object) -> str:
    if not isinstance(value, str):
        raise ValueError("URL must be a string")
    url = urlsplit(value)
    if (url.scheme != "https" or url.hostname not in ALLOWED_HOSTS
            or url.username or url.password or url.port not in (None, 443)
            or url.fragment or url.query):
        raise ValueError("Use an exact credential-free HTTPS asset URL on an approved upstream host")
    return value


def verify_rights(path: Path) -> tuple[dict, Path]:
    receipt = read_object(path)
    if receipt.get("source_id") != SOURCE_ID or receipt.get("schema_version") != "1.0.0":
        raise ValueError("Wrong rights receipt source or schema")
    if receipt.get("approved_for_acquisition") is not True:
        raise ValueError("Acquisition requires an explicit positive assessment")
    if not isinstance(receipt.get("public_redistribution_approved"), bool):
        raise ValueError("Redistribution must be assessed separately as a boolean")
    for key in ("licence", "approval_basis", "reviewed_by", "reviewed_on"):
        if not isinstance(receipt.get(key), str) or not receipt[key].strip():
            raise ValueError(f"Missing rights evidence field: {key}")
    if receipt["licence"].strip().lower() in {"unknown", "unverified", "pending"}:
        raise ValueError("Unverified terms cannot approve acquisition")
    date.fromisoformat(receipt["reviewed_on"])
    checked_url(receipt.get("payload_url"))
    checked_url(receipt.get("terms_url"))
    relative = receipt.get("terms_path")
    if not isinstance(relative, str) or not relative or Path(relative).is_absolute():
        raise ValueError("terms_path must be a non-empty relative path")
    base = path.resolve().parent
    terms = (base / relative).resolve()
    if not terms.is_relative_to(base) or not terms.is_file():
        raise ValueError("Terms evidence must be a file within the receipt directory")
    body = terms.read_bytes()
    if not body or hashlib.sha256(body).hexdigest() != receipt.get("terms_sha256"):
        raise ValueError("Missing or mismatched terms evidence bytes")
    return receipt, terms


def validate_pack(pack: Path = PACK) -> list[str]:
    errors: list[str] = []
    metadata = read_object(pack / "source_metadata.json")
    manifest = read_object(pack / "medallion.json")
    register = read_object(pack / "heor_register.json")
    if metadata.get("source_id") != SOURCE_ID or manifest.get("source_id") != SOURCE_ID:
        errors.append("Source identities disagree")
    for name in ("existing_assets", "additional_data_needs", "questions"):
        rows = register.get(name)
        if not isinstance(rows, list) or not rows or any(not isinstance(row, dict) for row in rows):
            errors.append(f"Invalid collection: {name}")
            continue
        ids = [row.get("id") for row in rows]
        if any(not isinstance(item, str) or not item for item in ids) or len(set(ids)) != len(ids):
            errors.append(f"Duplicate or missing IDs: {name}")
    if errors:
        return errors
    assets = {row["id"] for row in register["existing_assets"]}
    needs = {row["id"] for row in register["additional_data_needs"]}
    for question in register["questions"]:
        for field in ("question", "estimand", "design", "analysis_gate", "decision_output", "status"):
            if not question.get(field):
                errors.append(f"{question['id']}: missing {field}")
        for field, known in (("existing_asset_ids", assets), ("additional_data_ids", needs)):
            refs = question.get(field)
            if not isinstance(refs, list) or not refs or any(ref not in known for ref in refs):
                errors.append(f"{question['id']}: unresolved {field}")
    layers = manifest.get("layers", {})
    if set(layers) != {"bronze", "silver", "gold", "platinum"}:
        errors.append("Exactly four named medallion layers are required")
    if manifest.get("empirical_rows_ingested") == 0:
        if any(layer.get("status") != "blocked" for layer in layers.values()):
            errors.append("No empirical observations: data layers cannot be qualified")
    if manifest.get("publication", {}).get("enabled") is True:
        if metadata.get("rights", {}).get("public_redistribution_approved") is not True:
            errors.append("Publication lacks explicit redistribution approval")
    if manifest.get("publication", {}).get("new_repository") is not False:
        errors.append("Reuse existing repositories")
    return errors


def acquire(receipt_path: Path, output: Path, root: Path = ROOT) -> Path:
    receipt, terms = verify_rights(receipt_path)
    target = output.resolve()
    bronze = (root / ".open-social-data/youth-suicide/bronze").resolve()
    if target == bronze or not target.is_relative_to(bronze):
        raise ValueError("Output must be a new packet directory under .open-social-data/youth-suicide/bronze")
    if target.exists():
        raise ValueError("Refusing to overwrite an existing packet")
    archiver = root / "scripts/archive_public_url.py"
    if not archiver.is_file():
        raise ValueError("Existing shared archive_public_url.py is required")
    subprocess.run(
        [sys.executable, str(archiver), "--source-id", SOURCE_ID,
         "--url", receipt["payload_url"], "--terms-url", receipt["terms_url"],
         "--output", str(target)], check=True, timeout=150,
    )
    packet = read_object(target / "manifest.json")
    if packet.get("source_id") != SOURCE_ID:
        raise ValueError("Captured packet source identity mismatch")
    status = packet.get("receipt", {}).get("status")
    if not isinstance(status, int) or isinstance(status, bool) or not 200 <= status < 300:
        raise ValueError("Source did not return a successful HTTP receipt")
    if packet.get("archive_status") != "captured":
        raise ValueError("Source capture unavailable; retain disposition without promoting")
    checked_url(packet.get("receipt", {}).get("url"))
    payload = target / "payload"
    payload_info = packet.get("payload") or {}
    if not payload.is_file() or not payload.stat().st_size:
        raise ValueError("Captured packet has no non-empty payload")
    if payload.stat().st_size != payload_info.get("bytes"):
        raise ValueError("Payload byte-count mismatch; do not promote")
    if hashlib.sha256(payload.read_bytes()).hexdigest() != payload_info.get("sha256"):
        raise ValueError("Payload checksum mismatch; do not promote")
    shutil.copyfile(terms, target / "terms-evidence")
    evidence = dict(receipt, terms_path="terms-evidence")
    (target / "rights-receipt.json").write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf-8")
    # Keep the shared source manifest unchanged; qualification is separate evidence.
    qualification = {
        "status": "bronze-captured-not-silver-qualified", "source_id": SOURCE_ID,
        "rights_receipt": "rights-receipt.json", "public_redistribution_approved": receipt["public_redistribution_approved"],
        "access_script_sha256": hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
        "shared_archiver_sha256": hashlib.sha256(archiver.read_bytes()).hexdigest(),
        "non_claim": "No CSV schema, completeness, statistical validity or downstream analysis has been qualified.",
    }
    (target / "qualification.json").write_text(json.dumps(qualification, indent=2) + "\n", encoding="utf-8")
    with (target / "checksums.sha256").open("a", encoding="utf-8") as stream:
        for name in ("terms-evidence", "rights-receipt.json", "qualification.json"):
            stream.write(f"{hashlib.sha256((target / name).read_bytes()).hexdigest()}  {name}\n")
    return target


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("plan", help="Show current staged state; never makes network requests")
    commands.add_parser("check", help="Validate research and medallion contracts offline")
    fetch = commands.add_parser("acquire", help="Delegate explicitly approved acquisition to the shared archiver")
    fetch.add_argument("--rights-receipt", required=True, type=Path)
    fetch.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    try:
        if args.command == "plan":
            print(json.dumps(read_object(PACK / "medallion.json"), indent=2))
        elif args.command == "check":
            errors = validate_pack()
            print(json.dumps({"ok": not errors, "errors": errors}, indent=2))
            return int(bool(errors))
        else:
            print(acquire(args.rights_receipt, args.output))
    except (ValueError, OSError, subprocess.SubprocessError) as error:
        print(f"Blocked: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
