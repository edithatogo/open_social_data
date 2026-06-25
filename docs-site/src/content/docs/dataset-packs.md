---
title: Dataset Packs
description: Structure, required files, and conventions for creating dataset packs. Use templates from the /templates/ directory.
---

# Dataset Packs

A dataset pack is a self-contained directory under `datasets/<source>/<dataset_identifier>/` that includes metadata, documentation, fetch scripts, and data output.

## Standard Directory Layout

```
datasets/
  <source_acronym>/
    <dataset_identifier>/
      README.md              # Dataset overview (required)
      SESSION_LOG.md          # Session log for this dataset (required)
      source_metadata.json    # Metadata for source-backed packs (optional)
      data/
        *.parquet             # Fetched Parquet output files
        archive/              # Older versions (optional)
      docs/
        data_dictionary.md    # Variable descriptions (required)
        accessible_guide.md   # Plain-language guide (required)
      scripts/
        *.py                  # Fetch/validate scripts (required)
      logs/
        *.log                 # Processing logs
```

### Example: ABS Consumer Price Index

```
datasets/abs/consumer_price_index/
  README.md
  SESSION_LOG.md
  docs/
    data_dictionary.md
    accessible_guide.md
  scripts/
    fetch_consumer_price_index.py
```

### Example: AIHW MyHospitals (multiple measures)

```
datasets/aihw/myhospitals/
  README.md
  SESSION_LOG.md
  data/
    aihw_myhospitals_MYH-ADM_20250708_041751.parquet
    aihw_myhospitals_MYH-ES_20250708_045925.parquet
    ...
  docs/
    data_dictionary_admissions.md
    data_dictionary_elective_surgery.md
    accessible_guide_admissions.md
    accessible_guide_elective_surgery.md
    ...
  scripts/
    fetch_aihw_myhospitals_data.py
    validate_myhospitals_data.py
    example_queries.py
  logs/
    myhospitals_processing.log
```

## Required Files

### README.md

Every dataset pack must have a `README.md` that covers:

- Dataset name, source agency, and update frequency
- Access method (API, download, or CLI-backed)
- Links to data dictionary and accessible guide
- Methodology notes and licence information
- Known caveats or suppression rules

Use the template at `templates/dataset_readme_template.md`.

### data_dictionary.md

A detailed description of every column in the dataset:

- Column name and type
- Allowed values or ranges
- Nullability and suppression rules
- Source-specific codes or classifications

Use the template at `templates/data_dictionary_template.md`. For AIHW MyHospitals, separate dictionaries per measure category are named `data_dictionary_<measure>.md`.

### accessible_guide.md

A plain-language guide that explains:

- What the dataset measures
- Why it matters
- How to interpret key indicators
- Limitations and caveats

Use the template at `templates/accessible_guide_template.md`.

### SESSION_LOG.md

Records the fetch and processing history for the dataset, including dates, row counts, and any issues encountered. Use `SESSION_LOG_TEMPLATE.md` as a starting point.

### source_metadata.json (optional)

For packs backed by structured source metadata, include a `source_metadata.json` file with fields such as:

```json
{
  "id": "CPI",
  "name": "Consumer Price Index",
  "description": "Measures quarterly changes in the price of a fixed basket of goods and services.",
  "version": "1.0",
  "source_url": "https://example.test/cpi",
  "update_frequency": "quarterly",
  "licence": "Creative Commons Attribution 4.0"
}
```

Currently used by:
- `datasets/abs/average_weekly_earnings/source_metadata.json`
- `datasets/moh/nz_health_survey_annual_update/source_metadata.json`
- `datasets/stats_nz/household_income_housing_costs/source_metadata.json`

## Fetch Scripts

Each dataset pack should have at least one Python fetch script in its `scripts/` directory. These scripts typically:

1. Call the Rust CLI or directly invoke the provider API
2. Save the output as Parquet in the `data/` directory
3. Log progress to a file in `logs/`

### Pattern for CLI-backed fetch (ABS)

```python
# datasets/abs/consumer_price_index/scripts/fetch_consumer_price_index.py
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "data" / "cpi.parquet"

result = subprocess.run([
    "cargo", "run", "--bin", "open-social-data-cli", "--",
    "fetch", "abs", "CPI",
    "--output", str(OUTPUT),
], capture_output=True, text=True)

if result.returncode != 0:
    print(result.stderr, file=sys.stderr)
    sys.exit(1)
print(result.stdout)
```

### Pattern for API-backed fetch (AIHW)

```python
# datasets/aihw/myhospitals/scripts/fetch_aihw_myhospitals_data.py
import pandas as pd
import requests

response = requests.get(
    "https://myhospitals-api.aihw.gov.au/api/v1/measure/MYH-ADM",
    headers={"Accept": "application/json"}
)
data = response.json()
df = pd.json_normalize(data)
df.to_parquet("data/aihw_myhospitals_MYH-ADM_<date>.parquet")
```

## Creating a New Dataset Pack

1. **Choose a source acronym** (`abs`, `stats_nz`, `aihw`, `moh`, etc.)
2. **Create the directory structure** under `datasets/<source>/<dataset_identifier>/`
3. **Copy and fill in templates** from `templates/`
4. **Write the fetch script** in `scripts/`
5. **Run the fetch** and verify the output Parquet
6. **Run validation:**
   ```bash
   cargo run --bin open-social-data-cli -- validate dataset-packs
   ```
7. **Record the session** in `SESSION_LOG.md`

## Validation

The `validate dataset-packs` CLI command checks for:

- `README.md` and `SESSION_LOG.md` presence
- `docs/data_dictionary*.md` and `docs/accessible_guide*.md` presence
- At least one `*.py` script in `scripts/`

The Python `maintenance_check.py` script performs additional checks:

```cmd
python scripts\maintenance_check.py
```

This checks for placeholder markers, broken URLs (with `--live` flag), and Python AST syntax validity.