//! Data ingestion, schema validation, and Parquet export pipeline.
//!
//! Provides [`RawRecord`] and [`RecordBatchBuilder`] for building DataFrames
//! from raw data, [`validate_schema`] for runtime schema checks, and
//! [`write_parquet_atomic`] for crash-safe Parquet file writes.

use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use polars::prelude::*;

use crate::error::{CoreError, Result};

/// A single raw record with named field values.
#[derive(Debug, Clone, Default)]
pub struct RawRecord {
    pub fields: HashMap<String, String>,
}

impl RawRecord {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.fields.insert(name.into(), value.into());
        self
    }
}

/// Collects RawRecords and builds a Polars DataFrame.
#[derive(Debug, Clone)]
pub struct RecordBatchBuilder {
    records: Vec<RawRecord>,
    schema_keys: Vec<String>,
}

impl RecordBatchBuilder {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            schema_keys: Vec::new(),
        }
    }

    pub fn push(&mut self, record: RawRecord) {
        let mut keys = record.fields.keys().collect::<Vec<_>>();
        keys.sort();
        for key in keys {
            if !self.schema_keys.contains(key) {
                self.schema_keys.push(key.clone());
            }
        }
        self.records.push(record);
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn build(&self) -> Result<DataFrame> {
        if self.records.is_empty() {
            return Ok(DataFrame::default());
        }

        let mut columns = Vec::new();
        for key in &self.schema_keys {
            let values: Vec<&str> = self
                .records
                .iter()
                .map(|record| record.fields.get(key).map(String::as_str).unwrap_or(""))
                .collect();
            columns.push(Series::new(key.into(), values).into());
        }
        DataFrame::new(self.records.len(), columns)
            .map_err(|e| CoreError::TransformationError(e.to_string()))
    }
}

impl Default for RecordBatchBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedColumn {
    pub name: String,
    pub dtype: DataType,
}

impl ExpectedColumn {
    pub fn new(name: impl Into<String>, dtype: DataType) -> Self {
        Self {
            name: name.into(),
            dtype,
        }
    }
}

pub fn validate_schema(frame: &DataFrame, expected: &[ExpectedColumn]) -> Result<()> {
    if frame.width() != expected.len() {
        return Err(CoreError::TransformationError(format!(
            "schema width mismatch: expected {}, found {}",
            expected.len(),
            frame.width()
        )));
    }

    for column in expected {
        let series = frame.column(&column.name).map_err(|_| {
            CoreError::TransformationError(format!("missing column {}", column.name))
        })?;
        if series.dtype() != &column.dtype {
            return Err(CoreError::TransformationError(format!(
                "column {} has type {:?}, expected {:?}",
                column.name,
                series.dtype(),
                column.dtype
            )));
        }
    }

    Ok(())
}

pub fn write_parquet_atomic(frame: &DataFrame, output_path: impl AsRef<Path>) -> Result<()> {
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp_path = tmp_path_for(output_path);
    if tmp_path.exists() {
        fs::remove_file(&tmp_path)?;
    }

    let mut file = File::create(&tmp_path)?;
    let mut frame = frame.clone();
    ParquetWriter::new(&mut file)
        .finish(&mut frame)
        .map_err(|error| CoreError::TransformationError(error.to_string()))?;
    drop(file);

    if output_path.exists() {
        fs::remove_file(output_path)?;
    }
    fs::rename(&tmp_path, output_path)?;
    Ok(())
}

/// Reads a DataFrame from a Parquet file path.
pub fn read_parquet(path: impl AsRef<Path>) -> Result<DataFrame> {
    let file = File::open(path.as_ref())?;
    ParquetReader::new(file)
        .finish()
        .map_err(|e| CoreError::TransformationError(e.to_string()))
}

fn tmp_path_for(output_path: &Path) -> PathBuf {
    let mut name = output_path
        .file_name()
        .map(|file_name| file_name.to_os_string())
        .unwrap_or_else(|| "output.parquet".into());
    name.push(".tmp");
    output_path.with_file_name(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_exact_schema() {
        let frame = DataFrame::new(
            2,
            vec![
                Series::new("id".into(), &[1_i64, 2]).into(),
                Series::new("name".into(), &["A", "B"]).into(),
            ],
        )
        .unwrap();

        let expected = vec![
            ExpectedColumn::new("id", DataType::Int64),
            ExpectedColumn::new("name", DataType::String),
        ];

        validate_schema(&frame, &expected).unwrap();
    }

    #[test]
    fn rejects_missing_column() {
        let frame = DataFrame::new(1, vec![Series::new("id".into(), &[1_i64]).into()]).unwrap();
        let expected = vec![
            ExpectedColumn::new("id", DataType::Int64),
            ExpectedColumn::new("name", DataType::String),
        ];

        assert!(validate_schema(&frame, &expected).is_err());
    }

    #[test]
    fn parquet_atomic_write_creates_file() {
        let frame =
            DataFrame::new(3, vec![Series::new("x".into(), &[1_i64, 2, 3]).into()]).unwrap();

        let tmp = std::env::temp_dir().join(format!("test_atomic_{}", std::process::id()));
        let out_path = tmp.join("output.parquet");

        write_parquet_atomic(&frame, &out_path).unwrap();
        assert!(out_path.exists(), "output file should exist");
        assert!(
            !out_path.with_extension("parquet.tmp").exists(),
            "tmp file should be cleaned up"
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn read_parquet_roundtrip() {
        let frame =
            DataFrame::new(3, vec![Series::new("x".into(), &[1_i64, 2, 3]).into()]).unwrap();

        let tmp = std::env::temp_dir().join(format!("test_roundtrip_{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let path = tmp.join("test.parquet");

        write_parquet_atomic(&frame, &path).unwrap();
        let loaded = read_parquet(&path).unwrap();
        assert_eq!(loaded.height(), 3);
        assert_eq!(loaded.width(), 1);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn record_batch_builder_produces_dataframe() {
        let mut builder = RecordBatchBuilder::new();
        builder.push(RawRecord::new().with("id", "1").with("name", "Alice"));
        builder.push(RawRecord::new().with("id", "2").with("name", "Bob"));

        let df = builder.build().unwrap();
        assert_eq!(df.height(), 2);
        assert_eq!(df.width(), 2);
    }

    #[test]
    fn record_batch_builder_empty() {
        let builder = RecordBatchBuilder::new();
        assert!(builder.is_empty());
        let df = builder.build().unwrap();
        assert_eq!(df.height(), 0);
    }
}
