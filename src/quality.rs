//! Data quality assertions and validation reports.
//!
//! Defines [`QualityAssertion`] variants (non-null, unique, range, etc.),
//! runs them over DataFrames via [`validate_quality`], and persists
//! [`QualityReport`]s atomically. Also provides [`DeltaUpdater`] for
//! incremental delta updates to existing Parquet datasets.

use std::collections::HashSet;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use polars::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QualityAssertion {
    NonNull {
        column: String,
    },
    NullLimit {
        column: String,
        max_nulls: usize,
    },
    Unique {
        column: String,
    },
    NumericRange {
        column: String,
        min: Option<f64>,
        max: Option<f64>,
    },
    AllowedValues {
        column: String,
        values: Vec<String>,
    },
}

impl QualityAssertion {
    pub fn non_null(column: impl Into<String>) -> Self {
        Self::NonNull {
            column: column.into(),
        }
    }

    pub fn null_limit(column: impl Into<String>, max_nulls: usize) -> Self {
        Self::NullLimit {
            column: column.into(),
            max_nulls,
        }
    }

    pub fn unique(column: impl Into<String>) -> Self {
        Self::Unique {
            column: column.into(),
        }
    }

    pub fn numeric_range(column: impl Into<String>, min: Option<f64>, max: Option<f64>) -> Self {
        Self::NumericRange {
            column: column.into(),
            min,
            max,
        }
    }

    pub fn allowed_values<I, S>(column: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::AllowedValues {
            column: column.into(),
            values: values.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityIssue {
    pub column: String,
    pub assertion: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualityReport {
    pub checked_rows: usize,
    pub issues: Vec<QualityIssue>,
}

impl QualityReport {
    pub fn is_valid(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn into_result(self) -> Result<Self> {
        if self.is_valid() {
            Ok(self)
        } else {
            Err(CoreError::QualityError(
                self.issues
                    .iter()
                    .map(|issue| issue.message.as_str())
                    .collect::<Vec<_>>()
                    .join("; "),
            ))
        }
    }

    pub fn save_atomic(&self, path: impl AsRef<Path>) -> Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let tmp_path = tmp_path_for(path);
        if tmp_path.exists() {
            fs::remove_file(&tmp_path)?;
        }

        let writer = BufWriter::new(File::create(&tmp_path)?);
        serde_json::to_writer_pretty(writer, self)?;

        if path.exists() {
            fs::remove_file(path)?;
        }
        fs::rename(tmp_path, path)?;
        Ok(())
    }
}

pub fn validate_quality(
    frame: &DataFrame,
    assertions: &[QualityAssertion],
) -> Result<QualityReport> {
    let mut issues = Vec::new();

    for assertion in assertions {
        match assertion {
            QualityAssertion::NonNull { column } => {
                let series = frame.column(column).map_err(|_| {
                    CoreError::QualityError(format!("missing required column {column}"))
                })?;
                let null_count = series.null_count();
                if null_count > 0 {
                    issues.push(QualityIssue {
                        column: column.clone(),
                        assertion: "non_null".to_string(),
                        message: format!("{column} contains {null_count} null value(s)"),
                    });
                }
            }
            QualityAssertion::NullLimit { column, max_nulls } => {
                let series = frame.column(column).map_err(|_| {
                    CoreError::QualityError(format!("missing required column {column}"))
                })?;
                let null_count = series.null_count();
                if null_count > *max_nulls {
                    issues.push(QualityIssue {
                        column: column.clone(),
                        assertion: "null_limit".to_string(),
                        message: format!(
                            "{column} contains {null_count} null value(s), above limit {max_nulls}"
                        ),
                    });
                }
            }
            QualityAssertion::Unique { column } => {
                let series = frame.column(column).map_err(|_| {
                    CoreError::QualityError(format!("missing required column {column}"))
                })?;
                let unique_count = unique_stringified_count(series)?;
                let non_null_count = frame.height().saturating_sub(series.null_count());
                if unique_count < non_null_count {
                    issues.push(QualityIssue {
                        column: column.clone(),
                        assertion: "unique".to_string(),
                        message: format!(
                            "{column} contains {} duplicate non-null value(s)",
                            non_null_count - unique_count
                        ),
                    });
                }
            }
            QualityAssertion::NumericRange { column, min, max } => {
                let series = frame.column(column).map_err(|_| {
                    CoreError::QualityError(format!("missing required column {column}"))
                })?;
                let numeric = series.cast(&DataType::Float64).map_err(|error| {
                    CoreError::QualityError(format!(
                        "{column} could not be cast to Float64: {error}"
                    ))
                })?;
                let values = numeric.f64().map_err(|error| {
                    CoreError::QualityError(format!("{column} is not Float64 after cast: {error}"))
                })?;

                let below_min = min
                    .map(|min| {
                        values
                            .clone()
                            .into_iter()
                            .flatten()
                            .filter(|value| *value < min)
                            .count()
                    })
                    .unwrap_or(0);
                let above_max = max
                    .map(|max| {
                        values
                            .clone()
                            .into_iter()
                            .flatten()
                            .filter(|value| *value > max)
                            .count()
                    })
                    .unwrap_or(0);

                if below_min > 0 || above_max > 0 {
                    issues.push(QualityIssue {
                        column: column.clone(),
                        assertion: "numeric_range".to_string(),
                        message: format!(
                            "{column} has {below_min} value(s) below min and {above_max} value(s) above max"
                        ),
                    });
                }
            }
            QualityAssertion::AllowedValues { column, values } => {
                let series = frame.column(column).map_err(|_| {
                    CoreError::QualityError(format!("missing required column {column}"))
                })?;
                let allowed = values.iter().map(String::as_str).collect::<Vec<_>>();
                let invalid = series
                    .str()
                    .map_err(|error| {
                        CoreError::QualityError(format!("{column} is not a string column: {error}"))
                    })?
                    .clone()
                    .into_iter()
                    .flatten()
                    .filter(|value| !allowed.contains(value))
                    .count();

                if invalid > 0 {
                    issues.push(QualityIssue {
                        column: column.clone(),
                        assertion: "allowed_values".to_string(),
                        message: format!("{column} contains {invalid} unexpected value(s)"),
                    });
                }
            }
        }
    }

    Ok(QualityReport {
        checked_rows: frame.height(),
        issues,
    })
}

fn unique_stringified_count(series: &Column) -> Result<usize> {
    let mut values = HashSet::new();
    for index in 0..series.len() {
        let value = series
            .get(index)
            .map_err(|error| CoreError::QualityError(error.to_string()))?;
        if !value.is_null() {
            values.insert(value.to_string());
        }
    }
    Ok(values.len())
}

fn tmp_path_for(path: &Path) -> PathBuf {
    let mut name = path
        .file_name()
        .map(|file_name| file_name.to_os_string())
        .unwrap_or_else(|| "quality-report.json".into());
    name.push(".tmp");
    path.with_file_name(name)
}

pub fn provider_payload_assertions() -> Vec<QualityAssertion> {
    vec![
        QualityAssertion::non_null("provider"),
        QualityAssertion::non_null("dataset_id"),
        QualityAssertion::non_null("payload_bytes"),
        QualityAssertion::numeric_range("payload_bytes", Some(0.0), None),
    ]
}

// ---------------------------------------------------------------------------
// DeltaUpdater – incremental / delta append to Parquet datasets
// ---------------------------------------------------------------------------

/// Handles incremental (delta) updates to existing Parquet datasets.
///
/// # Usage
///
/// ```ignore
/// use open_social_data::quality::DeltaUpdater;
///
/// // First call writes a new file; subsequent calls append.
/// DeltaUpdater::append_to_parquet(&new_data, &existing_path, &output_path)?;
/// ```
pub struct DeltaUpdater;

impl DeltaUpdater {
    /// Appends `new_data` to an existing Parquet file at `existing_path`,
    /// writing the combined result to `output_path`.
    ///
    /// If `existing_path` does **not** exist, the function simply writes
    /// `new_data` as a fresh file (first-time write).
    pub fn append_to_parquet(
        new_data: &DataFrame,
        existing_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> Result<()> {
        let existing_path = existing_path.as_ref();
        let output_path = output_path.as_ref();

        if existing_path.exists() {
            let existing_df = crate::pipeline::read_parquet(existing_path)?;
            let combined = existing_df.vstack(new_data).map_err(|e| {
                CoreError::TransformationError(format!("delta append vstack failed: {e}"))
            })?;
            crate::pipeline::write_parquet_atomic(&combined, output_path)?;
        } else {
            crate::pipeline::write_parquet_atomic(new_data, output_path)?;
        }

        Ok(())
    }

    /// Appends `new_data` after validating its schema matches `expected_schema`.
    ///
    /// This is a convenience wrapper that calls [`validate_schema`] before
    /// delegating to [`append_to_parquet`].
    pub fn append_with_schema_check(
        new_data: &DataFrame,
        expected_schema: &[crate::pipeline::ExpectedColumn],
        existing_path: impl AsRef<Path>,
        output_path: impl AsRef<Path>,
    ) -> Result<()> {
        crate::pipeline::validate_schema(new_data, expected_schema)?;
        Self::append_to_parquet(new_data, existing_path, output_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_valid_assertions() {
        let frame = DataFrame::new(3, vec![
            Series::new("value".into(), &[1_i64, 2, 3]).into(),
            Series::new("state".into(), &["ok", "warn", "ok"]).into(),
        ])
        .unwrap();

        let report = validate_quality(
            &frame,
            &[
                QualityAssertion::non_null("value"),
                QualityAssertion::numeric_range("value", Some(0.0), Some(5.0)),
                QualityAssertion::allowed_values("state", ["ok", "warn"]),
            ],
        )
        .unwrap();

        assert!(report.is_valid());
        assert_eq!(report.checked_rows, 3);
    }

    #[test]
    fn reports_range_failures() {
        let frame =
            DataFrame::new(vec![Series::new("value".into(), &[1_i64, 9, 12]).into()]).unwrap();

        let report = validate_quality(
            &frame,
            &[QualityAssertion::numeric_range(
                "value",
                Some(0.0),
                Some(10.0),
            )],
        )
        .unwrap();

        assert_eq!(report.issues.len(), 1);
        assert!(!report.is_valid());
    }

    #[test]
    fn reports_uniqueness_failures() {
        let frame =
            DataFrame::new(vec![Series::new("id".into(), &["a", "b", "a"]).into()]).unwrap();

        let report = validate_quality(&frame, &[QualityAssertion::unique("id")]).unwrap();

        assert_eq!(report.issues.len(), 1);
        assert_eq!(report.issues[0].assertion, "unique");
    }

    // -----------------------------------------------------------------------
    // DeltaUpdater tests
    // -----------------------------------------------------------------------

    #[test]
    fn delta_updater_writes_new_file_when_no_existing() {
        let frame = DataFrame::new(vec![Series::new("x".into(), &[1_i64, 2]).into()]).unwrap();

        let tmp = std::env::temp_dir().join(format!("delta_new_{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let out = tmp.join("output.parquet");

        DeltaUpdater::append_to_parquet(&frame, &out, &out).unwrap();
        assert!(out.exists(), "output file should be created");

        // Verify the content
        let loaded = crate::pipeline::read_parquet(&out).unwrap();
        assert_eq!(loaded.height(), 2);
        assert_eq!(loaded.width(), 1);

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn delta_updater_appends_to_existing() {
        let frame1 = DataFrame::new(vec![Series::new("x".into(), &[1_i64, 2]).into()]).unwrap();
        let frame2 = DataFrame::new(vec![Series::new("x".into(), &[3_i64, 4]).into()]).unwrap();

        let tmp = std::env::temp_dir().join(format!("delta_append_{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let out = tmp.join("output.parquet");

        DeltaUpdater::append_to_parquet(&frame1, &out, &out).unwrap();
        DeltaUpdater::append_to_parquet(&frame2, &out, &out).unwrap();

        let loaded = crate::pipeline::read_parquet(&out).unwrap();
        assert_eq!(loaded.height(), 4, "two appends should produce 4 rows");

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn delta_updater_preserves_data_across_multiple_appends() {
        let batch1 = DataFrame::new(vec![
            Series::new("id".into(), &[1_i64, 2]).into(),
            Series::new("val".into(), &[10_i64, 20]).into(),
        ])
        .unwrap();
        let batch2 = DataFrame::new(vec![
            Series::new("id".into(), &[3_i64]).into(),
            Series::new("val".into(), &[30_i64]).into(),
        ])
        .unwrap();
        let batch3 = DataFrame::new(vec![
            Series::new("id".into(), &[4_i64, 5]).into(),
            Series::new("val".into(), &[40_i64, 50]).into(),
        ])
        .unwrap();

        let tmp = std::env::temp_dir().join(format!("delta_multi_{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let out = tmp.join("multi.parquet");

        DeltaUpdater::append_to_parquet(&batch1, &out, &out).unwrap();
        DeltaUpdater::append_to_parquet(&batch2, &out, &out).unwrap();
        DeltaUpdater::append_to_parquet(&batch3, &out, &out).unwrap();

        let loaded = crate::pipeline::read_parquet(&out).unwrap();
        assert_eq!(loaded.height(), 5, "three appends should produce 5 rows total");

        // Verify column is intact
        let ids: Vec<i64> = loaded
            .column("id")
            .unwrap()
            .i64()
            .unwrap()
            .into_iter()
            .map(|v| v.unwrap())
            .collect();
        assert_eq!(ids, vec![1, 2, 3, 4, 5], "ids should be sequentially preserved");

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn delta_updater_schema_check_rejects_mismatch() {
        let ok_frame = DataFrame::new(vec![
            Series::new("id".into(), &[1_i64]).into(),
            Series::new("name".into(), &["A"]).into(),
        ])
        .unwrap();

        let bad_frame = DataFrame::new(vec![
            Series::new("id".into(), &[2_i64]).into(),
            // missing "name" column, has "x" instead
            Series::new("x".into(), &["B"]).into(),
        ])
        .unwrap();

        let schema = &[
            crate::pipeline::ExpectedColumn::new("id", DataType::Int64),
            crate::pipeline::ExpectedColumn::new("name", DataType::String),
        ];

        let tmp = std::env::temp_dir().join(format!("delta_schema_{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let out = tmp.join("schema_test.parquet");

        // First append should pass
        DeltaUpdater::append_with_schema_check(&ok_frame, schema, &out, &out).unwrap();

        // Second append with mismatched schema should fail
        let result = DeltaUpdater::append_with_schema_check(&bad_frame, schema, &out, &out);
        assert!(
            result.is_err(),
            "schema mismatch should produce an error"
        );

        let _ = std::fs::remove_dir_all(&tmp);
    }

    #[test]
    fn delta_updater_empty_new_data_produces_empty_file() {
        let empty = DataFrame::new(vec![
            Series::new("x".into(), &[] as &[i64]).into(),
        ])
        .unwrap();

        let tmp = std::env::temp_dir().join(format!("delta_empty_{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let out = tmp.join("empty.parquet");

        DeltaUpdater::append_to_parquet(&empty, &out, &out).unwrap();

        let loaded = crate::pipeline::read_parquet(&out).unwrap();
        assert_eq!(loaded.height(), 0, "empty data should produce 0 rows");

        let _ = std::fs::remove_dir_all(&tmp);
    }
}
