# AIHW MyHospitals Data

**Source:** Australian Institute of Health and Welfare (AIHW)
**Original Source Link:** MyHospitals Website [https://www.aihw.gov.au/myhospitals](https://www.aihw.gov.au/myhospitals)
**API Documentation:** [https://myhospitalsapi.aihw.gov.au/index.html](https://myhospitalsapi.aihw.gov.au/index.html)
**API Swagger Spec:** [https://myhospitalsapi.aihw.gov.au/swagger/v1/swagger.json](https://myhospitalsapi.aihw.gov.au/swagger/v1/swagger.json)
**Date Accessed/Processed:** $(date +"%Y-%m-%d") (Initial exploration and script development)

---

## 1. Description

This dataset provides access to data from the Australian Institute of Health and Welfare's (AIHW) MyHospitals platform. MyHospitals reports on the performance of more than 1,000 public and private hospitals across Australia. It includes data on services, admissions, emergency department (ED) care, elective surgery, waiting times, safety and quality, and financial performance.

This repository aims to provide scripts to access and process data from the MyHospitals API, focusing initially on specific measure categories like Emergency Department (ED) Waiting Times.

---

## 2. Data Structure & Access

*   **Data Format:** The AIHW MyHospitals API provides data primarily in JSON format. Some endpoints also offer CSV or direct XLSX downloads.
*   **Data Access:** Data is accessed via the MyHospitals API using scripts in this repository.
*   **Data File(s) (Planned):** Processed data for specific measure categories will be stored in Parquet format, e.g., `data/aihw_myhospitals_ed_waits_YYYYMMDD_HHMMSS.parquet`.
*   **API Base URL:** `https://myhospitalsapi.aihw.gov.au/api/v1/`
*   **Key API Endpoints Used:**
    *   Metadata: `/measure-categories`, `/measures`, `/reporting-units`, etc.
    *   Data: `/flat-formatted-data-extract/{measure-category-code}`
*   **Scripts for Access:**
    *   `scripts/shared/aihw_api_fetcher.py` (for general API metadata interaction)
    *   `scripts/fetch_aihw_myhospitals_data.py` (for fetching and processing specific MyHospitals data)

---

## 3. Key Information & Variables

*Detailed information for each processed dataset will be available in its specific data dictionary and accessible guide. This initial setup focuses on "ED Waiting Times" (`MYH-ED-WAITS`).*

*   **Data Dictionary (ED Waiting Times):** [`docs/data_dictionary_ed_waits.md`](./docs/data_dictionary_ed_waits.md) (To be created/populated based on fetched data)
*   **Accessible Guide (ED Waiting Times):** [`docs/accessible_guide_ed_waits.md`](./docs/accessible_guide_ed_waits.md) (To be created/populated)

*Key variables within the "ED Waiting Times" (MYH-ED-WAITS) flat formatted data extract typically include:*
*   `reporting_unit_name`: Name of the hospital.
*   `measure_name`: Specific performance indicator (e.g., "Percentage of patients who commenced treatment within the recommended time").
*   `reported_measure_name`: Disaggregation of the measure (e.g., by triage category).
*   `data_period`: The period the data refers to (e.g., "2022-23").
*   `formatted_value`: The reported value, often a percentage or time.
*   `raw_value`: The numerical value.
*   Various geographical and peer group classifications.

---

## 4. Methodology & Collection

Data is collected by AIHW from various sources, including national health data collections and directly from hospitals or state/territory health authorities. Methodology varies by measure.

*   **Source Methodology:** Refer to the AIHW MyHospitals website under "About the data" or specific measure pages for detailed methodology. Example: [https://www.aihw.gov.au/myhospitals/about-the-data/emergency-department-care](https://www.aihw.gov.au/myhospitals/about-the-data/emergency-department-care)

---

## 5. Known Issues, Limitations, or Caveats

*   The API uses pagination for flat data extracts (max 1000 records per request). Scripts must handle this.
*   Data definitions and collection methods can change over time; users should consult AIHW documentation for specific interpretation.
*   Caveats and suppression rules apply to some data points, as indicated in the API response.
*   Data is typically for hospitals and may not cover all healthcare services.

---

## 6. License & Usage Terms

*   **Original Source License:** Creative Commons Attribution 3.0 Australia (CC BY 3.0 AU). See [https://www.aihw.gov.au/copyright](https://www.aihw.gov.au/copyright).
*   **Repository License for derived elements:** Scripts in this repository are typically under an MIT license (refer to the main `LICENSE` file for the repository).

---

## 7. Citation

**Please cite the original source (example):**
> Australian Institute of Health and Welfare. (Year). *MyHospitals*. AIHW, Australian Government. Retrieved [Date] from [MyHospitals URL or specific data page URL].

*Replace (Year), [Date], and URLs as appropriate. Check the AIHW website for their preferred citation format.*

**If using data/scripts from this repository, please also acknowledge this project.**

---

## 8. Last Updated in this Repository

*   **Date:** $(date +"%Y-%m-%d")
*   **Changes Made:** Initial setup of README for AIHW MyHospitals data, focusing on ED Waiting Times (`MYH-ED-WAITS`) as the first measure category.

---

## 9. Contact / Questions

*   For questions about the data in this repository: [Link to repository issues page or maintainer contact]
*   For questions about the original data: Australian Institute of Health and Welfare - [https://www.aihw.gov.au/contact-us](https://www.aihw.gov.au/contact-us)

---
*This README is for the AIHW MyHospitals dataset integration.*
