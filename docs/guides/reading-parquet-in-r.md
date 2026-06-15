# Reading Open Social Data Parquet Files in R

This guide explains how to load Parquet files produced by the Open Social
Data CLI using R.

## Prerequisites

Install the required packages:

```r
install.packages("arrow")
# or
install.packages("duckdb")
```

## Loading a Parquet File

### With arrow
```r
library(arrow)

df <- read_parquet("output.parquet")
print(df)
glimpse(df)
```

### With duckdb
```r
library(duckdb)

con <- dbConnect(duckdb())
df <- dbGetQuery(con, "SELECT * FROM read_parquet('output.parquet')")
print(df)
dbDisconnect(con)
```

## Filtering Data

```r
# arrow (dplyr)
library(dplyr)
filtered <- df %>% filter(provider == "abs")

# duckdb
con <- dbConnect(duckdb())
filtered <- dbGetQuery(con, "
  SELECT * FROM read_parquet('output.parquet')
  WHERE provider = 'abs'
")
dbDisconnect(con)
```

## Exporting to CSV

```r
write.csv(df, "output.csv", row.names = FALSE)
```

## Notes

- Parquet files are columnar and compressed, making them efficient for
  storage and fast for analytical queries.
- The schema is self-describing - no separate schema file needed.
