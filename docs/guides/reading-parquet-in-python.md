# Reading Open Social Data Parquet Files in Python

This guide explains how to load Parquet files produced by the Open Social Data CLI using common Python data tools.

## Prerequisites

Install one of the following libraries:

### Using pandas

```bash
pip install pandas pyarrow
```

### Using polars

```bash
pip install polars
```

## Loading a Parquet File

### With pandas

```python
import pandas as pd

df = pd.read_parquet("output.parquet")
print(df.head())
print(df.info())
```

### With polars

```python
import polars as pl

df = pl.read_parquet("output.parquet")
print(df)
print(df.describe())
```

## Filtering Data

```python
# pandas
filtered = df[df["provider"] == "abs"]

# polars
filtered = df.filter(pl.col("provider") == "abs")
```

## Exporting to CSV

```python
# pandas
df.to_csv("output.csv", index=False)

# polars
df.write_csv("output.csv")
```

## Notes

Parquet files are columnar and compressed, making them efficient for storage and fast for analytical queries. The schema is self-describing, so no separate schema file is needed for basic loading.
