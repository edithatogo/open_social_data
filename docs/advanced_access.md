# Advanced Access and Visualization

Advanced access tools should remain optional and should not make the core repository harder to validate.

## Prototype Decision

The first prototype is a static AIHW MyHospitals dashboard generated from local Parquet extracts:

```cmd
python scripts\build_myhospitals_dashboard.py
```

The command writes `docs/advanced/myhospitals_summary.html`. This keeps the prototype in-repo, static, and reproducible without requiring a web server or dashboard runtime.

## Cross-Dataset Metadata Needs

Before building broader dashboards or analysis helpers, dataset packs need consistent metadata for:

* Geography: country, state, territory, region, facility, and source-specific geographic codes.
* Period: reference period, period start/end, update cadence, and release date.
* Indicator: official indicator name, repository-normalized name, and source codelist.
* Unit: measure unit, denominator, currency, price basis, and adjustment type.
* Caveats: suppression, small-cell warnings, provisional status, revision flags, and quality notes.

## Tool Boundary

Keep static prototypes in this repository while they directly support documentation and examples. Move richer applications to a separate package or site when they require persistent services, authentication, large generated assets, or independent release cadence.
