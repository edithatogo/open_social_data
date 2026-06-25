# Medium-Term Dataset Candidate Backlog

This backlog records proposed Track 10 dataset additions. Candidates are prioritised by social value, source stability, licence clarity, and fetch feasibility.

## Implemented in Track 10

| Priority | Source | Dataset | Value | Access / Licence Notes | Status |
|---|---|---|---|---|---|
| 1 | Stats NZ | Household income and housing-cost statistics | Cost-of-living, income, housing affordability, wellbeing denominators | Stats NZ release pages and ADE exports; source licence applies | Implemented pack-level integration |
| 2 | ABS | Average Weekly Earnings, Australia | Wage and labour-market analysis | ABS API coverage verified; dataflow and DSD require live refresh confirmation | Implemented pack-level integration |
| 3 | New Zealand Ministry of Health | New Zealand Health Survey annual key results | Public health outcomes and equity reporting | Annual Data Explorer CSV export; CC BY 4.0 noted on source page | Implemented pack-level integration |

## Proposed Next Additions

| Priority | Source | Dataset | Value | Feasibility Notes |
|---|---|---|---|---|
| 4 | Stats NZ | Rental price indexes / selected price indexes | Housing cost pressure and inflation workflows | ADE or release-table access; metadata capture needed |
| 5 | Stats NZ | Household net worth statistics | Wealth and household balance-sheet analysis | Release tables available; survey caveats important |
| 6 | ABS | Regional Population | Population denominators for Australian social indicators | Likely ABS API or release-table path |
| 7 | ABS | Housing Occupancy and Costs | Housing affordability and tenure analysis | Latest release is historical; 2023-24 SIH output was not released |
| 8 | ABS | Education and Work | Education, study, qualification, and work transitions | Summary release available; microdata access is restricted |
| 9 | Ministry of Education NZ | School rolls | Education participation and demographic planning | Public source assessment required before pack implementation |

## Selection Notes

The first three candidates were selected because they complete the medium-term requirement for more Stats NZ and ABS coverage while adding a non-Stats NZ/ABS/AIHW source agency. The remaining candidates should be implemented only after source metadata, licence, and refresh paths are confirmed.
