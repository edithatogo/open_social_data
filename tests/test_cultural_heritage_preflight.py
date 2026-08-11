import importlib.util
from pathlib import Path


SCRIPT = Path(__file__).parents[1] / "scripts" / "validate_cultural_heritage_preflight.py"
SPEC = importlib.util.spec_from_file_location("heritage_preflight", SCRIPT)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def test_gazette_requires_both_source_and_preservation_credentials() -> None:
    assert MODULE.missing_requirements("new-zealand-gazette", {"HF_TOKEN": "x"}) == [
        "DIGITALNZ_API_KEY",
        "IA_ACCESS_KEY",
        "IA_SECRET_KEY",
    ]


def test_cenotaph_requires_inventory_url_and_approved_host() -> None:
    environment = {"HF_TOKEN": "x", "IA_ACCESS_KEY": "x", "IA_SECRET_KEY": "x"}
    assert MODULE.missing_requirements("auckland-museum-online-cenotaph", environment) == ["CENOTAPH_INVENTORY_URL"]
    assert MODULE.approved_url("auckland-museum-online-cenotaph", "https://api.aucklandmuseum.com/search/cenotaph/_search")
    assert not MODULE.approved_url("auckland-museum-online-cenotaph", "https://example.test/export")


def test_papers_past_url_is_limited_to_first_party_hosts() -> None:
    assert MODULE.approved_url("papers-past-pre-1945", "https://paperspast.natlib.govt.nz/bulk")
    assert MODULE.approved_url("papers-past-pre-1945", "https://natlib.govt.nz/open-data/bulk")
    assert not MODULE.approved_url("papers-past-pre-1945", "http://paperspast.natlib.govt.nz/bulk")
