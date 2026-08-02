from __future__ import annotations

import gzip
import importlib.util
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace
from unittest import mock

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "archive_arcgis_feature_layer", ROOT / "scripts/archive_arcgis_feature_layer.py"
)
assert SPEC is not None and SPEC.loader is not None
archive = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(archive)

PUBLISHER_SPEC = importlib.util.spec_from_file_location(
    "publish_hf_archive", ROOT / "scripts/publish_hf_archive.py"
)
assert PUBLISHER_SPEC is not None and PUBLISHER_SPEC.loader is not None
publisher = importlib.util.module_from_spec(PUBLISHER_SPEC)
PUBLISHER_SPEC.loader.exec_module(publisher)


class ArcGisArchiveTests(unittest.TestCase):
    def test_page_requests_use_form_encoded_post(self) -> None:
        response = mock.MagicMock()
        response.__enter__.return_value = response
        response.read.return_value = b'{"features":[]}'
        response.geturl.return_value = "https://services2.arcgis.com/example/query"
        response.status = 200
        response.headers.items.return_value = []
        with mock.patch.object(
            archive.urllib.request, "urlopen", return_value=response
        ) as opened:
            body, receipt = archive.request_bytes(
                "https://services2.arcgis.com/example/query",
                timeout=1,
                form={"objectIds": "10001,10002", "f": "json"},
            )
        request = opened.call_args.args[0]
        self.assertEqual(request.get_method(), "POST")
        self.assertEqual(request.data, b"objectIds=10001%2C10002&f=json")
        self.assertEqual(body, b'{"features":[]}')
        self.assertEqual(receipt["method"], "POST")
        self.assertEqual(
            receipt["request_body_sha256"], archive.sha256_bytes(request.data)
        )

    def test_hugging_face_revision_must_be_present(self) -> None:
        api = mock.MagicMock()
        api.repo_info.return_value = SimpleNamespace(sha="abc123")
        self.assertEqual(publisher.revision(api, "owner/dataset"), "abc123")
        api.repo_info.return_value = SimpleNamespace(sha=None)
        with self.assertRaisesRegex(SystemExit, "did not return a revision"):
            publisher.revision(api, "owner/dataset")

    def test_hugging_face_token_is_fail_closed(self) -> None:
        with (
            mock.patch.dict(publisher.os.environ, {}, clear=True),
            self.assertRaisesRegex(SystemExit, "HF_TOKEN is not configured"),
        ):
            publisher.api_client()

    def test_service_url_is_fail_closed(self) -> None:
        valid = "https://services2.arcgis.com/example/ArcGIS/rest/services/Test/FeatureServer/0"
        self.assertEqual(archive.validate_service_url(valid), valid)
        for invalid in (
            "http://services2.arcgis.com/example/FeatureServer/0",
            "https://example.com/FeatureServer/0",
            "https://services2.arcgis.com/example/FeatureServer/1",
            "https://services2.arcgis.com/example/FeatureServer/0?token=secret",
        ):
            with self.assertRaises(ValueError):
                archive.validate_service_url(invalid)

    def test_gzip_output_is_deterministic(self) -> None:
        payload = b'{"feature":1}\n'
        first = archive.deterministic_gzip(payload)
        second = archive.deterministic_gzip(payload)
        self.assertEqual(first, second)
        self.assertEqual(gzip.decompress(first), payload)

    def test_object_ids_reject_duplicates(self) -> None:
        with self.assertRaisesRegex(ValueError, "duplicate"):
            archive.object_ids(
                {"objectIdFieldName": "OBJECTID", "objectIds": [2, 1, 2]}, "fixture"
            )

    def test_page_must_match_frozen_inventory(self) -> None:
        payload = {
            "features": [
                {"attributes": {"OBJECTID": 1}, "geometry": {"rings": []}},
                {"attributes": {"OBJECTID": 2}, "geometry": None},
            ]
        }
        ids, nulls = archive.validate_page(
            payload, oid_field="OBJECTID", expected_ids=[1, 2], final_page=True
        )
        self.assertEqual(ids, [1, 2])
        self.assertEqual(nulls, 1)
        with self.assertRaisesRegex(ValueError, "frozen inventory"):
            archive.validate_page(
                payload, oid_field="OBJECTID", expected_ids=[2, 1], final_page=True
            )

    def test_arcgis_error_payload_is_rejected(self) -> None:
        with self.assertRaisesRegex(ValueError, "ArcGIS error"):
            archive.parse_arcgis_json(
                json.dumps({"error": {"code": 400, "message": "bad query"}}).encode(),
                "fixture",
            )

    def test_receipt_reports_missing_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            missing = Path(directory) / "missing.json"
            output = Path(directory) / "receipt.json"
            result = subprocess.run(
                [
                    sys.executable,
                    str(ROOT / "scripts/write_hf_archive_receipt.py"),
                    "--dataset",
                    "owner/dataset",
                    "--revision",
                    "abc123",
                    "--manifest",
                    str(missing),
                    "--output",
                    str(output),
                ],
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("unable to read archive manifest", result.stderr)
            self.assertFalse(output.exists())


if __name__ == "__main__":
    unittest.main()
