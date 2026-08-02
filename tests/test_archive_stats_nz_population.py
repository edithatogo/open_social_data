from __future__ import annotations

import importlib.util
import io
import tempfile
import unittest
import zipfile
from pathlib import Path
from unittest import mock

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "archive_stats_nz_population", ROOT / "scripts/archive_stats_nz_population.py"
)
assert SPEC is not None and SPEC.loader is not None
archive = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(archive)


def workbook_bytes(*, markers: tuple[str, ...] = archive.REQUIRED_MARKERS) -> bytes:
    output = io.BytesIO()
    with zipfile.ZipFile(output, "w") as workbook:
        workbook.writestr("[Content_Types].xml", "<Types/>")
        workbook.writestr("xl/workbook.xml", "<workbook/>")
        workbook.writestr(
            "xl/sharedStrings.xml",
            "<sst>" + "".join(f"<si><t>{value}</t></si>" for value in markers) + "</sst>",
        )
        for number in range(1, 8):
            workbook.writestr(f"xl/worksheets/sheet{number}.xml", "<worksheet/>")
    return output.getvalue()


class StatsNzPopulationArchiveTests(unittest.TestCase):
    def test_source_urls_are_exact_and_credential_free(self) -> None:
        self.assertEqual(
            archive.validate_stats_nz_url(archive.WORKBOOK_URL, archive.WORKBOOK_URL),
            archive.WORKBOOK_URL,
        )
        for invalid in (
            archive.WORKBOOK_URL.replace("https://", "http://"),
            archive.WORKBOOK_URL.replace("www.stats.govt.nz", "stats.govt.nz"),
            archive.WORKBOOK_URL + "?token=secret",
            archive.WORKBOOK_URL.replace("provisional.xlsx", "revised.xlsx"),
        ):
            with self.assertRaises(ValueError):
                archive.validate_stats_nz_url(invalid, archive.WORKBOOK_URL)

    def test_workbook_requires_exact_edition_markers_and_seven_sheets(self) -> None:
        result = archive.validate_workbook(workbook_bytes())
        self.assertEqual(result["worksheets"], 7)
        self.assertEqual(result["required_edition_markers"], list(archive.REQUIRED_MARKERS))
        with self.assertRaisesRegex(ValueError, "edition markers"):
            archive.validate_workbook(workbook_bytes(markers=archive.REQUIRED_MARKERS[:1]))

    def test_workbook_rejects_unsafe_or_corrupt_archives(self) -> None:
        output = io.BytesIO()
        with zipfile.ZipFile(output, "w") as workbook:
            workbook.writestr("../escape", "bad")
        with self.assertRaisesRegex(ValueError, "unsafe member path"):
            archive.validate_workbook(output.getvalue())
        with self.assertRaisesRegex(ValueError, "valid XLSX"):
            archive.validate_workbook(b"not-a-workbook")

    def test_capture_rejects_digest_drift_before_writing_final_packet(self) -> None:
        release = b"Subnational population estimates: At 30 June 2025"
        workbook = workbook_bytes()
        receipts = [
            {
                "url": archive.RELEASE_URL,
                "method": "GET",
                "status": 200,
                "retrieved_at": "2026-08-02T00:00:00Z",
                "headers": {"content-type": "text/html"},
            },
            {
                "url": archive.WORKBOOK_URL,
                "method": "GET",
                "status": 200,
                "retrieved_at": "2026-08-02T00:00:01Z",
                "headers": {"content-type": archive.EXPECTED_WORKBOOK_MEDIA_TYPE},
            },
        ]
        with tempfile.TemporaryDirectory() as directory, mock.patch.object(
            archive,
            "request_bytes",
            side_effect=[(release, receipts[0]), (workbook, receipts[1])],
        ):
            args = archive.parser().parse_args(
                [
                    "--output",
                    directory,
                    "--expected-bytes",
                    str(len(workbook)),
                    "--expected-sha256",
                    "0" * 64,
                ]
            )
            with self.assertRaisesRegex(ValueError, "SHA-256 changed"):
                archive.capture(args)
            self.assertFalse((Path(directory) / "latest.json").exists())


if __name__ == "__main__":
    unittest.main()
