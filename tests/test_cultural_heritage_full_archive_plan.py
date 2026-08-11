import json
from pathlib import Path


ROOT = Path(__file__).parents[1]
PLAN = ROOT / "config/acquisition/cultural-heritage-full-archive-plan.json"


def test_full_archive_plan_covers_requested_collections_without_duplicate_canonicals() -> None:
    payload = json.loads(PLAN.read_text(encoding="utf-8"))
    assert payload["canonical_repository"] == "edithatogo/open_social_data"
    assert "Internet Archive" in payload["redundancy_policy"]
    sources = {source["id"]: source for source in payload["sources"]}
    assert set(sources) == {
        "new-zealand-gazette",
        "auckland-museum-online-cenotaph",
        "papers-past-pre-1945",
    }
    assert all(source["redundant_target"] == "Internet Archive item series" for source in sources.values())
    assert all("status" in source for source in sources.values())
    assert all(source["tracking_issue"].startswith("https://github.com/edithatogo/open_social_data/issues/") for source in sources.values())
    assert all(source["blockers"] for source in sources.values())


def test_full_archive_plan_retains_release_boundaries() -> None:
    payload = json.loads(PLAN.read_text(encoding="utf-8"))
    sources = {source["id"]: source for source in payload["sources"]}
    assert "takedown" in sources["auckland-museum-online-cenotaph"]["release_rule"]
    assert "Māori" in sources["papers-past-pre-1945"]["release_rule"]
    assert "attribution" in sources["new-zealand-gazette"]["release_rule"]
    evidence = sources["auckland-museum-online-cenotaph"]["inventory_evidence"]
    assert evidence["exact_record_count"] == 271305
    assert evidence["normal_result_window_limit"] == 10000
    assert evidence["scroll_continuation"].startswith("POST /search/_search/scroll")
