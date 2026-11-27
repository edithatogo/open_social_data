"""Offline contract and negative-rights tests; fixtures contain no health observations."""
from __future__ import annotations

import hashlib
import importlib.util
import json
import shutil
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
PACK = ROOT / "datasets/pediatricsuicides/international_youth_suicide"
SPEC = importlib.util.spec_from_file_location("youth_access", PACK / "scripts/access.py")
assert SPEC is not None and SPEC.loader is not None
ACCESS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ACCESS)


class YouthSuicideContracts(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.base = Path(self.temp.name)

    def receipt(self, **changes: object) -> Path:
        terms = self.base / "terms.txt"
        terms.write_text("SYNTHETIC TEST TERMS, NOT A REAL DATA LICENCE", encoding="utf-8")
        obj = {
            "schema_version": "1.0.0", "source_id": ACCESS.SOURCE_ID,
            "approved_for_acquisition": True, "public_redistribution_approved": False,
            "licence": "synthetic-test-only", "approval_basis": "unit-test fixture only",
            "reviewed_by": "synthetic-test", "reviewed_on": "2026-09-10",
            "payload_url": "https://pediatricsuicides.ca/synthetic-test.csv",
            "terms_url": "https://pediatricsuicides.ca/synthetic-test-terms",
            "terms_path": "terms.txt", "terms_sha256": hashlib.sha256(terms.read_bytes()).hexdigest(),
        }
        obj.update(changes)
        path = self.base / "rights.json"
        path.write_text(json.dumps(obj), encoding="utf-8")
        return path

    def mutated_pack(self, name: str, transform) -> Path:
        pack = self.base / "pack"
        shutil.copytree(PACK, pack)
        path = pack / name
        obj = ACCESS.read_object(path)
        transform(obj)
        path.write_text(json.dumps(obj), encoding="utf-8")
        return pack

    def test_current_contract(self) -> None:
        self.assertEqual(ACCESS.validate_pack(), [])

    def test_exact_research_inventory(self) -> None:
        register = ACCESS.read_object(PACK / "heor_register.json")
        self.assertEqual(len(register["questions"]), 22)
        self.assertEqual(len(register["existing_assets"]), 13)
        self.assertEqual(len(register["additional_data_needs"]), 16)

    def test_duplicate_question_rejected(self) -> None:
        pack = self.mutated_pack("heor_register.json", lambda x: x["questions"].append(x["questions"][0]))
        self.assertTrue(ACCESS.validate_pack(pack))

    def test_unresolved_asset_rejected(self) -> None:
        pack = self.mutated_pack("heor_register.json", lambda x: x["questions"][0]["existing_asset_ids"].append("missing"))
        self.assertTrue(ACCESS.validate_pack(pack))

    def test_unresolved_need_rejected(self) -> None:
        pack = self.mutated_pack("heor_register.json", lambda x: x["questions"][0]["additional_data_ids"].append("missing"))
        self.assertTrue(ACCESS.validate_pack(pack))

    def test_fifth_layer_rejected(self) -> None:
        pack = self.mutated_pack("medallion.json", lambda x: x["layers"].update({"diamond": {"status": "blocked"}}))
        self.assertTrue(ACCESS.validate_pack(pack))

    def test_false_data_promotion_rejected(self) -> None:
        pack = self.mutated_pack("medallion.json", lambda x: x["layers"]["gold"].update({"status": "qualified"}))
        self.assertTrue(ACCESS.validate_pack(pack))

    def test_unlicensed_publication_rejected(self) -> None:
        pack = self.mutated_pack("medallion.json", lambda x: x["publication"].update({"enabled": True}))
        self.assertTrue(ACCESS.validate_pack(pack))

    def test_acquisition_and_redistribution_are_separate(self) -> None:
        receipt, _ = ACCESS.verify_rights(self.receipt())
        self.assertIs(receipt["public_redistribution_approved"], False)

    def test_false_or_string_approval_rejected(self) -> None:
        for value in (False, "true", None):
            with self.subTest(value=value), self.assertRaises(ValueError):
                ACCESS.verify_rights(self.receipt(approved_for_acquisition=value))

    def test_hash_mismatch_rejected(self) -> None:
        with self.assertRaises(ValueError):
            ACCESS.verify_rights(self.receipt(terms_sha256="0" * 64))

    def test_unverified_licence_rejected(self) -> None:
        with self.assertRaises(ValueError):
            ACCESS.verify_rights(self.receipt(licence="UNVERIFIED"))

    def test_unsafe_url_rejected(self) -> None:
        for url in ("http://pediatricsuicides.ca/x", "https://127.0.0.1/x",
                    "https://user:secret@pediatricsuicides.ca/x", "https://pediatricsuicides.ca/x?token=secret"):
            with self.subTest(url=url), self.assertRaises(ValueError):
                ACCESS.verify_rights(self.receipt(payload_url=url))

    def test_traversal_rejected(self) -> None:
        with self.assertRaises(ValueError):
            ACCESS.verify_rights(self.receipt(terms_path="../outside.txt"))

    def test_unsafe_output_never_calls_network(self) -> None:
        with patch.object(ACCESS.subprocess, "run") as run:
            with self.assertRaises(ValueError):
                ACCESS.acquire(self.receipt(), self.base / "data", root=self.base)
            run.assert_not_called()

    def test_existing_packet_is_immutable(self) -> None:
        target = self.base / ".open-social-data/youth-suicide/bronze/test"
        target.mkdir(parents=True)
        with patch.object(ACCESS.subprocess, "run") as run:
            with self.assertRaises(ValueError):
                ACCESS.acquire(self.receipt(), target, root=self.base)
            run.assert_not_called()

    def fake_capture(self, target: Path, **changes: object) -> None:
        target.mkdir(parents=True)
        payload = b"SYNTHETIC BYTE FIXTURE; NOT MORTALITY DATA\n"
        (target / "payload").write_bytes(payload)
        packet = {
            "source_id": ACCESS.SOURCE_ID, "archive_status": "captured",
            "receipt": {"status": 200, "url": "https://pediatricsuicides.ca/synthetic-test.csv"},
            "payload": {"sha256": hashlib.sha256(payload).hexdigest(), "bytes": len(payload)},
        }
        packet.update(changes)
        (target / "manifest.json").write_text(json.dumps(packet), encoding="utf-8")
        (target / "checksums.sha256").write_text("", encoding="utf-8")

    def prepare_archiver(self) -> None:
        (self.base / "scripts").mkdir(exist_ok=True)
        (self.base / "scripts/archive_public_url.py").write_text("# SYNTHETIC TEST STUB\n", encoding="utf-8")

    def test_successful_capture_preserves_rights_and_shared_manifest(self) -> None:
        self.prepare_archiver()
        target = self.base / ".open-social-data/youth-suicide/bronze/mock"
        with patch.object(ACCESS.subprocess, "run", side_effect=lambda *a, **k: self.fake_capture(target)) as run:
            self.assertEqual(ACCESS.acquire(self.receipt(), target, root=self.base), target)
            run.assert_called_once()
        qualification = ACCESS.read_object(target / "qualification.json")
        self.assertEqual(qualification["status"], "bronze-captured-not-silver-qualified")
        self.assertFalse(qualification["public_redistribution_approved"])
        self.assertNotIn("rights_receipt", ACCESS.read_object(target / "manifest.json"))
        self.assertEqual(len((target / "checksums.sha256").read_text().splitlines()), 3)

    def test_bad_capture_never_gets_qualification(self) -> None:
        self.prepare_archiver()
        cases = [
            {"source_id": "wrong"},
            {"receipt": {"status": 403, "url": "https://pediatricsuicides.ca/x"}},
            {"payload": {"bytes": 1, "sha256": "0" * 64}},
            {"payload": {"bytes": 40, "sha256": "0" * 64}},
            {"receipt": {"status": 200, "url": "https://127.0.0.1/x"}},
        ]
        for index, changes in enumerate(cases):
            target = self.base / f".open-social-data/youth-suicide/bronze/bad-{index}"
            with self.subTest(index=index):
                with patch.object(ACCESS.subprocess, "run", side_effect=lambda *a, **k: self.fake_capture(target, **changes)):
                    with self.assertRaises(ValueError):
                        ACCESS.acquire(self.receipt(), target, root=self.base)
                self.assertFalse((target / "qualification.json").exists())

    def test_native_dataset_validators(self) -> None:
        for name, function in (("validate_source_metadata", "validate_metadata"),
                               ("validate_dataset_packs", "validate_dataset")):
            spec = importlib.util.spec_from_file_location(name, ROOT / f"scripts/{name}.py")
            assert spec is not None and spec.loader is not None
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            self.assertEqual(getattr(module, function)(PACK), [])


if __name__ == "__main__":
    unittest.main()
