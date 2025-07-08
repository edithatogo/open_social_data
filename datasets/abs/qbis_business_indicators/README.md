# Business Indicators, Australia (QBIS)

**Source:** Australian Bureau of Statistics (ABS)
**Original Source Link:** [https://www.abs.gov.au/statistics/economy/business-indicators/business-indicators-australia](https://www.abs.gov.au/statistics/economy/business-indicators/business-indicators-australia) (General landing page)
**API Documentation:** [https://www.abs.gov.au/statistics/understanding-statistics/statistical-data-and-metadata-standards/abs-data-api-user-guide](https://www.abs.gov.au/statistics/understanding-statistics/statistical-data-and-metadata-standards/abs-data-api-user-guide)
**Date Accessed/Processed:** $(date +"%Y-%m-%d") (Note: Live data not yet fetched, DSD access pending)

---

## 1. Description

This dataset provides quarterly estimates of key economic indicators for the private sector in Australia. It includes data on sales, wages and salaries, company gross operating profits, and inventories. The data is often used to understand business performance and economic activity across various industries. This dataset was formerly known by catalogue number 5676.0.

The dataflow identifier for the ABS API is `QBIS`.

---

## 2. Data Structure & Access

*   **Data Format:** The ABS API provides data in SDMX-JSON format. This repository aims to process this into Parquet files.
*   **Data Access:** Data will be fetched using scripts in this repository.
*   **Data File(s):** Processed data will be stored in `data/qbis_business_indicators.parquet` (planned). Raw API responses might be temporarily stored in `data/raw/`.
*   **API Endpoint (for data):** `https://api.abs.gov.au/data/QBIS`
*   **API Endpoint (for Data Structure Definition - DSD):** `https://api.abs.gov.au/datastructure/ABS/QBIS/latest?detail=full&references=children`
*   **Scripts for Access:** `scripts/shared/abs_api_fetcher.py` (for general API interaction, including DSD) and `scripts/fetch_qbis_data.py` (to be created for specific data fetching and processing).

---

## 3. Key Information & Variables

*Detailed information is available in the data dictionary and accessible guide.*

*   **Data Dictionary:** [`docs/data_dictionary.md`](./docs/data_dictionary.md) (Explains variables, types, and meanings - currently a template, will be populated post-data-access)
*   **Accessible Guide:** [`docs/accessible_guide.md`](./docs/accessible_guide.md) (Explains the dataset in plain language - currently a template)

*Expected Key Variables (based on ABS descriptions, subject to confirmation via DSD):*
*   **Indicators:** Company Gross Operating Profits, Wages and Salaries, Sales of Goods and Services, Inventories.
*   **Dimensions:** Industry (ANZSIC division/subdivision), Time Period (Quarter), Data Type (e.g., Current Prices, Chain Volume Measures), Seasonal Adjustment (Original, Seasonally Adjusted, Trend), State/Territory.
*   **Measure:** Observed value (e.g., AUD millions).

---

## 4. Methodology & Collection

The data is compiled from the ABS Quarterly Business Indicators Survey (QBIS). This survey collects data from a sample of private sector businesses in Australia.

*   **Source Methodology:** [https://www.abs.gov.au/methodologies/business-indicators-australia-methodology](https://www.abs.gov.au/methodologies/business-indicators-australia-methodology) (Link to the latest methodology page on the ABS website)

---

## 5. Known Issues, Limitations, or Caveats

*   **Network Access:** Current development environment has intermittent issues accessing `api.abs.gov.au`, hindering live DSD and data fetching.
*   **DSD Details:** The exact structure (dimensions, codes, attributes) is based on general ABS API knowledge and website information, pending successful DSD retrieval.
*   Data is subject to revisions by the ABS.
*   Users should refer to ABS publications for detailed analysis, interpretations, and any specific caveats related to data collection periods or events (e.g., COVID-19 impacts).
*   The scope is generally the private sector; refer to ABS documentation for specifics on industry inclusions/exclusions.

---

## 6. License & Usage Terms

*   **Original Source License:** The ABS website states data is generally available under a Creative Commons Attribution license. Users should verify the specific terms on the ABS website. See: [https://www.abs.gov.au/about/website-privacy-copyright-and-disclaimer#copyright-and-creative-commons](https://www.abs.gov.au/about/website-privacy-copyright-and-disclaimer#copyright-and-creative-commons)
*   **Repository License for derived elements:** Scripts in this repository are typically under an MIT license (refer to the main `LICENSE` file for the repository).

---

## 7. Citation

**Please cite the original source (example):**
> Australian Bureau of Statistics. (Year). *Business Indicators, Australia* (Catalogue No. 5676.0 if referring to historical context, or by title for current). Canberra: ABS. Retrieved from [ABS website URL]

*Replace (Year) and specific URL as appropriate. Check the ABS website for their preferred citation format.*

**If using data/scripts from this repository, please also acknowledge this project.**

---

## 8. Last Updated in this Repository

*   **Date:** $(date +"%Y-%m-%d")
*   **Changes Made:** Initial population of README for QBIS dataset. Structure created.

---

## 9. Contact / Questions

*   For questions about the data in this repository: [Link to repository issues page or maintainer contact]
*   For questions about the original data: Australian Bureau of Statistics - [https://www.abs.gov.au/about/contact-us](https://www.abs.gov.au/about/contact-us)

---
*This README is for the ABS Business Indicators (QBIS) dataset.*
