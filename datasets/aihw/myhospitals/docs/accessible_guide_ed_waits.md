# Accessible Guide to MyHospitals - Emergency Department (ED) Waiting Times

**Understanding ED Waiting Times in Plain Language**

**Source:** Australian Institute of Health and Welfare (AIHW) - MyHospitals (`MYH-ED-WAITS` measure category)
**Guide Updated:** $(date +"%Y-%m-%d")

---

## 1. What is This Data About?

*   **In simple terms, what does this dataset tell us?**
    This dataset shows how long people wait for care in public hospital Emergency Departments (EDs) across Australia. It looks at things like the percentage of patients seen on time according to their urgency (triage category) and how long patients spend in the ED overall.
*   **Who collects this information and why?**
    The Australian Institute of Health and Welfare (AIHW) collects and reports this information on their MyHospitals website. They do this to provide transparency about hospital performance and help people understand how hospitals are doing in this key area of care.
*   **What time period does it cover?**
    The data typically covers financial years (e.g., 2021-22, 2022-23) and is updated periodically by AIHW.
*   **How often is it updated?**
    AIHW updates MyHospitals data regularly, often annually for many measures. Check the AIHW website for specific update cycles.

---

## 2. Why is This Data Important?

Understanding ED waiting times is important because it reflects how accessible emergency care is and how well hospitals are managing patient flow during busy periods.

*   **What kind of questions can this data help answer?**
    *   What percentage of patients at a specific hospital are seen within the recommended time for their urgency level (e.g., resuscitation, emergency, urgent)?
    *   How does my local hospital compare to other similar hospitals (peer group) or the national average for ED waiting times?
    *   Has ED performance changed over time for a particular hospital or state?
    *   How long do patients typically spend in the ED before being admitted to the hospital, transferred, or sent home?
*   **Who might find this data useful?**
    *   **Patients and the Public:** To get an idea of ED performance at their local hospitals.
    *   **Hospitals and Health Services:** To monitor their own performance, identify areas for improvement, and compare against peers.
    *   **Governments and Policymakers:** To understand system pressures and inform health policy and funding.
    *   **Researchers and Media:** To analyze trends in healthcare delivery.

---

## 3. How Can You Use This Data? (Simple Examples)

*   **Example 1: Percentage Seen on Time**
    *   You might look up "Hospital X" and find that for "Urgent" patients (Triage Category 3), 70% were seen by a doctor or nurse within the recommended 30 minutes. The data might also show a peer group average of 75%, suggesting Hospital X is slightly below its peers for this measure.
*   **Example 2: Length of Stay in ED**
    *   The data could show that at "Hospital Y", 50% of patients who were subsequently discharged home completed their ED visit within 3 hours and 30 minutes.

---

## 4. Key Things to Know Before Using This Data

*   **Triage Categories Matter:** ED patients are sorted by urgency (triage). "Resuscitation" (life-threatening) cases should be seen immediately, while "Non-urgent" cases can wait longer. The data is often broken down by these categories.
*   **Not All Hospitals are the Same:** Hospitals are grouped into "peer groups" (e.g., major hospitals, children's hospitals) because they handle different types of patients and have different resources. Comparing a small rural hospital to a large city teaching hospital directly might not be fair without considering these peer groups.
*   **Data Can Be Complex:** Some measures look at percentages seen on time, others look at the time by which a certain percentage of patients are seen (e.g., 50th percentile, 90th percentile). Read the `measure_name` carefully.
*   **"Formatted Value" vs. "Raw Value":** The data often provides a `formatted_value` (e.g., "70%") and a `raw_value` (e.g., 70.0). The formatted value is usually easier to read.
*   **Caveats are Important:** Sometimes data points have `caveats` (notes or symbols) indicating special circumstances, data quality issues, or why a value might be different. These should be considered.
*   **Focus on Trends and Comparisons:** A single number for one hospital at one point in time might not tell the whole story. Look for trends over time or compare with similar hospitals or averages.

---

## 5. Where to Find More Information

*   **For detailed explanations of every data item:** See the [`data_dictionary_ed_waits.md`](./data_dictionary_ed_waits.md).
*   **For how to download or access the data files from this repository:** See the main MyHospitals [`README.md`](../README.md).
*   **For official information from the source:** Visit the Australian Institute of Health and Welfare MyHospitals website: [https://www.aihw.gov.au/myhospitals](https://www.aihw.gov.au/myhospitals) and look for information on Emergency Department Care.

---
*This guide aims to make the AIHW MyHospitals ED Waiting Times data easier to understand.*
