import json
from pathlib import Path


ROOT = Path(__file__).parents[1]
INVENTORY = ROOT / "config/acquisition/public-source-inventory.json"


def test_inventory_is_fail_closed_until_archived() -> None:
    payload = json.loads(INVENTORY.read_text(encoding="utf-8"))
    assert payload["policy"] == "discovery-only-until-raw-packet-and-rights-receipt-exist"
    sources = payload["sources"]
    assert {source["issue"] for source in sources} == {36, 37}
    assert all(source["archive_status"] == "not-archived" for source in sources)
    assert all("non_claim" in source for source in sources)


def test_issue_37_has_network_and_regional_gtfs_candidates() -> None:
    payload = json.loads(INVENTORY.read_text(encoding="utf-8"))
    issue_sources = [source for source in payload["sources"] if source["issue"] == 37]
    assert {source["id"] for source in issue_sources} >= {
        "nzta-amds-network-model",
        "metro-christchurch-static-gtfs",
        "auckland-transport-static-gtfs",
    }


def test_issue_36_has_independent_food_retail_families() -> None:
    payload = json.loads(INVENTORY.read_text(encoding="utf-8"))
    issue_sources = [source for source in payload["sources"] if source["issue"] == 36]
    assert {source["id"] for source in issue_sources} >= {
        "osm-food-retail-nz",
        "auckland-food-premises-register",
    }
