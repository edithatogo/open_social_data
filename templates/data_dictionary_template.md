# Data Dictionary for [Dataset Name]

**Dataset Source:** [e.g., Statistics New Zealand (Stats NZ) - Specific Survey/Collection Name]
**Date of Dictionary Creation/Update:** [YYYY-MM-DD]

---

## 1. Introduction

*Briefly explain the purpose of this data dictionary. It should help users understand the structure, content, and meaning of each variable within the [Dataset Name] dataset.*
*Mention the data format(s) this dictionary pertains to (e.g., "This dictionary describes the variables in the CSV files provided for this dataset.").*

---

## 2. Variables

*Use a table to describe each variable (column) in the dataset. Add or remove columns to the table template below as needed (e.g., you might add "Source Variable Name" if you've renamed variables from the original source).*

| Variable Name     | Description                                   | Data Type     | Allowed Values / Format / Range | Units         | Missing Values | Notes / Source Definition                               |
|-------------------|-----------------------------------------------|---------------|---------------------------------|---------------|----------------|---------------------------------------------------------|
| `variable_id_1`   | *Clear, concise description of the variable.* | *e.g., Integer, String, Float, Boolean, Date* | *e.g., "Male", "Female"; "YYYY-MM-DD"; "0-100"; ISO 3166-1 alpha-2 codes* | *e.g., NZD, %, Count, Kilograms* | *e.g., "NA", "999", "Blank"* | *Any additional notes, specific definitions from the source agency, or calculation methods.* |
| `variable_name_2` | *Description of what this variable represents.* | *e.g., Text*  | *Free text, or specific categories like "Urban", "Rural"* | *N/A*         | *e.g., "NULL"* | *If this is a derived variable, explain how it was derived.* |
| `variable_date_3` | *The date to which this record or observation pertains.* | *e.g., Date*  | *ISO 8601 format: YYYY-MM-DD*   | *N/A*         | *e.g., Blank*  | *Clarify if it's an event date, reporting date, etc.*   |
| `value_x`         | *The measured or reported value.*             | *e.g., Numeric (Decimal)* | *Positive numbers only*         | *e.g., Dollars* | *e.g., "-99"*  | *Specify precision if relevant.*                        |
| `category_code`   | *A code representing a specific category.*    | *e.g., Integer* | *See Section 3 for code list mapping.* | *N/A*         | *e.g., 0*      | *Codes provided by source agency.*                      |
| ...               | ...                                           | ...           | ...                             | ...           | ...            | ...                                                     |

---

## 3. Code Lists / Classifications (if applicable)

*If any variables use codes or specific classification systems, detail them here. This section can be repeated for multiple coded variables.*

### 3.1. [Variable Name Using Codes, e.g., `region_code`] - [Name of Classification, e.g., Regional Council Codes 2023]

*Link to official source for the classification if available.*

| Code  | Label / Description        |
|-------|----------------------------|
| `01`  | *Northland Region*         |
| `02`  | *Auckland Region*          |
| ...   | ...                        |

### 3.2. [Another Variable Using Codes] - [Name of Classification]

| Code  | Label / Description        |
|-------|----------------------------|
| `A`   | *Category A*               |
| `B`   | *Category B*               |
| ...   | ...                        |

---

## 4. Further Information

*Include links to any external documentation, glossaries, or resources from the source agency that provide further context for understanding the data variables.*

*   [Link to Source Agency Glossary]
*   [Link to Source Agency Classification Standards]

---

*This data dictionary template is intended to be filled out for each dataset. It should reside in the specific dataset's directory, e.g., `datasets/stats_nz/my_dataset_name/data_dictionary.md`.*
