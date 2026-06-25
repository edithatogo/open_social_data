# Data Dictionary for Average Weekly Earnings, Australia

**Dataset Source:** Australian Bureau of Statistics  
**Date of Dictionary Creation/Update:** 2026-06-23

## Introduction

This dictionary describes the expected row-level representation after ABS SDMX-JSON data is parsed by the Rust ABS provider.

## Variables

| Variable Name | Description | Data Type | Allowed Values / Format / Range | Units | Missing Values | Notes / Source Definition |
|---|---|---|---|---|---|---|
| `dataset_id` | ABS dataflow identifier. | String | `AWE` once confirmed | N/A | N/A | Added by provider output. |
| `time_period` | Reference period. | String | ABS period labels | N/A | Blank | Preserve ABS time labels. |
| `measure` | Earnings measure. | String | Source codelist labels | N/A | Blank | Examples may include full-time adult ordinary time earnings or total earnings. |
| `sex` | Sex category where supplied. | String | ABS codelist values | N/A | Blank | Use source labels and codes. |
| `state_or_territory` | Geography dimension where supplied. | String | ABS state and territory values | N/A | Blank | Do not mix with national totals without filtering. |
| `sector` | Public/private/all-sector category where supplied. | String | ABS codelist values | N/A | Blank | Optional dimension depending on table. |
| `value` | Published earnings estimate. | Numeric | Non-negative values | AUD | Suppressed/blank | Check source notes for seasonality and scope. |
| `unit` | Unit of measure. | String | AUD, index, percent where applicable | N/A | Blank | Required before comparing measures. |

## Code Lists / Classifications

Exact codelists require live DSD inspection. Expected codelists include time period, measure, sex, geography, sector, and unit of measure.

## Further Information

* ABS Indicator API: https://www.abs.gov.au/statistics/application-programming-interfaces-apis/indicator-api
* ABS citation guidance: https://www.abs.gov.au/how-cite-abs-sources
