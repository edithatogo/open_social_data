# Data Dictionary for Business Indicators, Australia (QBIS)

**Dataset Source:** Australian Bureau of Statistics (ABS) - Quarterly Business Indicators Survey
**Date of Dictionary Creation/Update:** 2026-06-18 (local wrapper added; exact codelists pending live DSD confirmation)

---

## 1. Introduction

This data dictionary describes the variables within the "Business Indicators, Australia" (`QBIS`) dataset, as processed and stored in this repository as Parquet. The data originates from the Australian Bureau of Statistics (ABS) API.

The purpose of this document is to help users understand the structure, content, and meaning of each variable. The repository has a CLI-backed QBIS fetch wrapper and mocked SDMX row-level parsing tests. Due to intermittent access to the live ABS endpoint from this workspace, exact codelists and attributes remain based on ABS publications and common SDMX patterns until direct DSD inspection is possible.

---

## 2. Variables

*The following table is a preliminary list of expected variables. Exact names, codes, and types are subject to confirmation.*

| Variable Name        | Description                                                                 | Data Type     | Allowed Values / Format / Range                                  | Units            | Missing Values | Notes / Source Definition                                      |
|----------------------|-----------------------------------------------------------------------------|---------------|------------------------------------------------------------------|------------------|----------------|----------------------------------------------------------------|
| `TIME_PERIOD`        | The reference period for the observation (Quarter).                         | `String`      | `YYYY-Qn` (e.g., "2023-Q1")                                      | N/A              | e.g., Blank    | Represents a calendar quarter.                                 |
| `INDICATOR`          | Code representing the specific business indicator being measured.           | `String`      | e.g., `CGOP` (Company Gross Operating Profits), `WAGES_SAL` (Wages and Salaries), `SALES_GS` (Sales of Goods and Services), `INVENTORIES` (Inventories) - *Actual codes TBC* | N/A              | e.g., Blank    | See Section 3 for potential code list mapping (TBC).           |
| `INDUSTRY`           | Code for the Australian and New Zealand Standard Industrial Classification (ANZSIC) division or subdivision. | `String`      | ANZSIC codes (e.g., "A", "B01", "TOTAL") - *Actual codes TBC*    | N/A              | e.g., Blank    | See Section 3 for potential code list mapping (TBC).           |
| `REGION`             | Code representing the State or Territory.                                   | `String`      | e.g., "AUS", "NSW", "VIC" - *Actual codes TBC*                   | N/A              | e.g., Blank    | Standard ABS State/Territory codes. "AUS" for Australia total. |
| `DATA_TYPE`          | The type of data value, e.g., related to price valuation.                 | `String`      | e.g., `CP` (Current Prices), `CVM` (Chain Volume Measures) - *Actual codes TBC* | N/A              | e.g., Blank    | Specifies the economic valuation of the measure.               |
| `SEAS_ADJ`           | Seasonal adjustment type.                                                   | `String`      | e.g., `SA` (Seasonally Adjusted), `ORIG` (Original), `TREND` - *Actual codes TBC* | N/A              | e.g., Blank    | Indicates the type of seasonal adjustment applied.             |
| `OBS_VALUE`          | The observed value for the indicator and its dimensions.                    | `Float`       | Numeric value.                                                   | e.g., AUD Millions | e.g., NaN, Null | The primary measure. Unit specified by `UNIT_MEASURE` attribute. |
| `UNIT_MEASURE`       | Attribute: Unit of measure for `OBS_VALUE`.                                 | `String`      | e.g., "AUDM" (Australian Dollars, Millions) - *Actual codes TBC* | N/A              | e.g., Blank    | Attribute related to `OBS_VALUE`.                              |
| `OBS_STATUS`         | Attribute: Observation status flag.                                         | `String`      | e.g., `F` (Final), `P` (Preliminary), `R` (Revised) - *Actual codes TBC* | N/A              | e.g., Blank    | Attribute providing context on the data point's lifecycle.     |
| `FREQ`               | Attribute or Dimension: Frequency of the data.                              | `String`      | `Q` (Quarterly)                                                  | N/A              | e.g., Blank    | QBIS data is quarterly.                                        |
| ...                  | *Other potential dimensions or attributes as per DSD.*                      | ...           | ...                                                              | ...              | ...            | ...                                                            |

---

## 3. Code Lists / Classifications (Preliminary - To Be Confirmed)

*This section will be populated with actual codelists once the DSD is successfully retrieved and parsed. The following are examples of expected codelists based on ABS standards.*

### 3.1. `INDICATOR` - Business Activity Indicators

*Link to official source for the classification if available (once identified).*

| Code (Example) | Label / Description                     |
|----------------|-----------------------------------------|
| `A2100303F`    | *Example: Company Gross Operating Profits* |
| `A2100304W`    | *Example: Wages and Salaries*             |
| `A2100305X`    | *Example: Sales of Goods and Services*    |
| `A2100306Y`    | *Example: Inventories*                    |
| ...            | ...                                     |

### 3.2. `INDUSTRY` - ANZSIC 2006 (Aggregates or Divisions)

*Link to official source: [https://www.abs.gov.au/ausstats/abs@.nsf/Latestproducts/1292.0Main%20Features2006%20(Revision%202.0)](https://www.abs.gov.au/ausstats/abs@.nsf/Latestproducts/1292.0Main%20Features2006%20(Revision%202.0))*

| Code (Example) | Label / Description                               |
|----------------|---------------------------------------------------|
| `A`            | *Agriculture, Forestry and Fishing*               |
| `B`            | *Mining*                                          |
| `TOTAL`        | *All selected industries (or similar aggregate)*  |
| ...            | ...                                               |

### 3.3. `REGION` - ABS Standard Geographical Classification

| Code (Example) | Label / Description        |
|----------------|----------------------------|
| `AUS`          | *Australia*                |
| `1`            | *New South Wales*          |
| `2`            | *Victoria*                 |
| ...            | ...                        |

*(Note: Actual ABS codes for states might be different, e.g., "NSW", "VIC")*

### 3.4. `DATA_TYPE` - Price Valuation

| Code (Example) | Label / Description        |
|----------------|----------------------------|
| `1`            | *Current Prices*           |
| `2`            | *Chain Volume Measures*    |
| ...            | ...                        |

### 3.5. `SEAS_ADJ` - Seasonal Adjustment

| Code (Example) | Label / Description        |
|----------------|----------------------------|
| `10`           | *Original*                 |
| `20`           | *Seasonally Adjusted*      |
| `30`           | *Trend*                    |
| ...            | ...                        |

---

## 4. Further Information

*   **ABS Website - Business Indicators, Australia:** [https://www.abs.gov.au/statistics/economy/business-indicators/business-indicators-australia](https://www.abs.gov.au/statistics/economy/business-indicators/business-indicators-australia)
*   **ABS Methodology:** [https://www.abs.gov.au/methodologies/business-indicators-australia-methodology](https://www.abs.gov.au/methodologies/business-indicators-australia-methodology)
*   **ABS Data API User Guide:** [https://www.abs.gov.au/statistics/understanding-statistics/statistical-data-and-metadata-standards/abs-data-api-user-guide](https://www.abs.gov.au/statistics/understanding-statistics/statistical-data-and-metadata-standards/abs-data-api-user-guide)

---
*This data dictionary is preliminary and will be updated upon successful retrieval and parsing of the official ABS Data Structure Definition (DSD) for the QBIS dataflow.*
