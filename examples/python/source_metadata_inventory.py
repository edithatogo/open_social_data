"""Print a compact inventory of dataset source metadata files."""

from __future__ import annotations

import json
from pathlib import Path

import pandas as pd


ROOT = Path(__file__).resolve().parents[2]


def main() -> None:
    rows: list[dict[str, object]] = []
    for path in sorted((ROOT / "datasets").glob("*/*/source_metadata.json")):
        data = json.loads(path.read_text(encoding="utf-8"))
        rows.append(
            {
                "dataset": str(path.parent.relative_to(ROOT / "datasets")),
                "source_agency": data["source_agency"],
                "update_cadence": data["update_cadence"],
                "unit_count": len(data.get("units", [])),
                "codelist_count": len(data.get("codelists", [])),
            }
        )
    if not rows:
        raise SystemExit("No source_metadata.json files found")
    print(pd.DataFrame(rows).to_string(index=False))


if __name__ == "__main__":
    main()
