# HEOR questions and data map

The complete machine-readable specification is [`../heor_register.json`](../heor_register.json). It contains each estimand, design, existing-asset reference, additional-data requirement, join contract, access route, ownership, analysis gate and decision output. None is analysis-ready merely because it is listed.

## Questions

| ID | Question | Existing assets | Additional data |
|---|---|---|---|
| H01 | Do child benefits and refundable tax credits reduce youth suicide? | A01, A02, A03, A04, A05, A09 | N01, N03, N04, N12, N15 |
| H02 | Do minimum wages and unemployment protections affect youth outcomes through family security? | A01, A02, A03, A04, A09 | N01, N03, N04, N12 |
| H03 | Do housing assistance and eviction protections improve youth outcomes? | A05, A09 | N01, N03, N04, N12 |
| H04 | Do social protections buffer youth mortality during economic shocks? | A03, A04, A05, A09 | N01, N03, N04, N12 |
| H05 | What are the age-specific health and economic consequences of youth suicide? | A01, A02, A03, A06, A10 | N01, N02, N06, N07, N15 |
| H06 | How are burden and prevention gains distributed? | A05, A06, A10 | N01, N05, N06, N07, N12, N15 |
| H07 | How do age compatibility, coding and incomplete coverage alter conclusions? | A06 | N01, N13 |
| H08 | What is the cost of delayed or revised mortality reporting? | A11 | N01, N08, N13, N15 |
| H09 | When is improved surveillance worth its cost? | A10, A11, A12 | N01, N05, N06, N07, N13, N15 |
| H10 | Which prevention portfolio gives greatest health gain under a defined budget? | A01, A02, A10, A11, A12 | N01, N02, N05, N06, N07, N12, N14, N15 |
| H11 | Is enhanced aftercare and care continuity cost-effective? | A07, A10, A12 | N05, N06, N07, N14, N15 |
| H12 | Can multidisciplinary/task-shared youth care improve access and value? | A10, A12 | N05, N06, N07, N14, N15 |
| H13 | What funding and workforce are needed to implement prevention at scale? | A01, A02, A07, A10, A12 | N05, N06, N14, N15 |
| H14 | When do government, health-system and societal rankings disagree? | A10, A11, A12 | N05, N06, N07, N12, N15 |
| H15 | Does timing school support around the school calendar improve outcomes? | A09, A10 | N01, N05, N06, N08, N09, N15 |
| H16 | What is the value of school-based prevention including educational outcomes? | A10, A12 | N05, N06, N07, N09, N12, N15 |
| H17 | How does bereavement affect parent employment, sibling education and family health? | A10 | N07, N11, N12, N15 |
| H18 | Is family/community postvention good value? | A10, A12 | N05, N06, N07, N11, N14, N15 |
| H19 | What are the benefits, harms and economic effects of age-based digital policies? | A09, A10 | N01, N04, N08, N10, N12, N15 |
| H20 | Which rural/remote delivery model reduces unmet need cost-effectively? | A07, A10, A12, A13 | N05, N06, N07, N12, N14, N16 |
| H21 | Do lower out-of-pocket costs or changed coverage improve youth outcomes? | A05, A09, A10 | N01, N03, N05, N06, N14, N16 |
| H22 | Which international results can support an ANZ or CHHHS decision? | A07, A09, A10, A11, A12 | N01, N05, N06, N07, N12, N14, N15 |

## Existing assets

| ID | Asset | Evidence status |
|---|---|---|
| A01 | ABS CPI | source_pack_present |
| A02 | Stats NZ CPI | source_pack_present |
| A03 | ABS labour force and average weekly earnings | source_pack_present |
| A04 | Stats NZ labour market statistics | source_pack_present |
| A05 | Stats NZ household income and housing costs | metadata_inspected |
| A06 | Stats NZ population estimates | source_pack_present |
| A07 | AIHW MyHospitals archive extracts | payload_paths_present_unvalidated |
| A08 | NZ Health Survey | source_pack_present |
| A09 | Health Policy Atlas ANZ | documented_capability |
| A10 | Value of Perspective / DCEA framework | documented_capability |
| A11 | voiage | documented_interface |
| A12 | Microcosting Health Workforce | documented_capability |
| A13 | Existing HF catalogue and source-packet archive | repository_descriptions_inspected |

## Additional requirements

| ID | Data requirement | Current status |
|---|---|---|
| N01 | Mortality exports and original-source validation | identified_not_acquired |
| N02 | Age-specific survival and life tables | new_source_discovery_required |
| N03 | Dated policy and simulated entitlement panel | source_family_identified |
| N04 | Macroeconomic and social conditions | source_family_identified |
| N05 | Intervention effects and implementation evidence | new_evidence_review_required |
| N06 | Resource use and unit costs | partly_supported_by_existing_cost_logic |
| N07 | Utilities, wellbeing and family outcomes | new_evidence_review_required |
| N08 | High-frequency mortality and reporting revisions | source_family_identified |
| N09 | School calendars and education exposure | new_source_discovery_required |
| N10 | Digital-policy exposure and implementation | new_source_discovery_required |
| N11 | Linked family and sibling outcomes | restricted_request_required |
| N12 | Equity strata and geographic concordances | new_data_qualification_required |
| N13 | Classification and coverage validation | new_custodian_or_validation_data_required |
| N14 | Actual service pathways, capacity and uptake | local_data_request_required |
| N15 | Economic decision conventions | decision_protocol_required |
| N16 | Access, travel and out-of-pocket burden | new_or_local_data_request_required |

## Priority and interpretation

The first protocol candidates are H01 (income protection), H05 (burden), H07 (measurement), H09 (surveillance VOI) and H10 (prevention portfolio). Start with data qualification, then choose a feasible causal design and defined decision. Priority is a proposed sequencing choice, not evidence of feasibility or a study result.

Public national mortality cannot establish family spillovers, individual treatment effects, school-day effects, local service capacity or exact age-threshold digital-policy effects. Linked-data projects remain in approved secure environments. Tests cover catalogue and acquisition contracts, not clinical or causal validity.
