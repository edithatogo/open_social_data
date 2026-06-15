use std::fs::{self, File};
use std::path::{Path, PathBuf};

use polars::prelude::*;

use crate::error::{CoreError, Result};

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
        let frame = DataFrame::new(vec![
            Series::new("id".into(), &[1_i64, 2]).into(),
            Series::new("name".into(), &["A", "B"]).into(),
        ])
        .unwrap();

        let expected = vec![
            ExpectedColumn::new("id", DataType::Int64),
            ExpectedColumn::new("name", DataType::String),
        ];

        validate_schema(&frame, &expected).unwrap();
    }

    #[test]
    fn rejects_missing_column() {
        let frame = DataFrame::new(vec![Series::new("id".into(), &[1_i64]).into()]).unwrap();
        let expected = vec![
            ExpectedColumn::new("id", DataType::Int64),
            ExpectedColumn::new("name", DataType::String),
        ];

        assert!(validate_schema(&frame, &expected).is_err());
    }
}
