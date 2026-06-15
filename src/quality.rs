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
                            .into_iter()
                            .flatten()
                            .filter(|value| *value < min)
                            .count()
                    })
                    .unwrap_or(0);
                let above_max = max
                    .map(|max| {
                        values
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_valid_assertions() {
        let frame = DataFrame::new(vec![
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
}
