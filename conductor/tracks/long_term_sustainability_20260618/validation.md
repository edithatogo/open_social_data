# Validation Evidence

## 2026-06-18

Commands:

```cmd
python scripts\maintenance_check.py
python scripts\build_myhospitals_dashboard.py
python scripts\validate_dataset_packs.py
python scripts\maintenance_check.py
git diff --check
```

Results:

* `python scripts\maintenance_check.py` - passed. It verified dataset pack completeness, source URL presence, stale placeholder markers, and Python helper syntax validation.
* `python scripts\build_myhospitals_dashboard.py` - passed and wrote `docs\advanced\myhospitals_summary.html`.
* `python scripts\validate_dataset_packs.py` - passed for all current ABS, Stats NZ, and AIHW dataset packs.
* `python scripts\maintenance_check.py` - passed.
* `git diff --check` - passed.

Notes:

* Live URL checking is implemented behind `python scripts\maintenance_check.py --live --timeout 15`, but is intentionally not required for CI because source agency endpoints can be temporarily unavailable.
* The first dashboard run exposed a Pandas future warning for all-empty columns; the loader now drops all-empty columns before concatenating local Parquet extracts.
