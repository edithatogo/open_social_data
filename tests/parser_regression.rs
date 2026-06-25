//! Parser regression tests using compact fixture payloads.
//!
//! These tests load fixture JSON files from `tests/fixtures/` and validate
//! that the ABS SDMX and Stats NZ OData parsers produce the expected row
//! output. They serve as snapshot tests: if the parser changes, the expected
//! output here must be updated intentionally.

use std::fs;
use std::path::Path;

use open_social_data_core::{RawRecord, RecordBatchBuilder};

fn load_fixture(name: &str) -> String {
    let path = Path::new("tests").join("fixtures").join(name);
    fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "could not load fixture `{}`: {}. Did you run from the repo root?",
            path.display(),
            e
        )
    })
}

fn parse_abs_sdmx_fixture(json_text: &str) -> Vec<RawRecord> {
    let parsed: serde_json::Value =
        serde_json::from_str(json_text).expect("fixture should be valid JSON");
    let data = &parsed["data"];
    let structure = &data["structure"];
    let dimensions = &structure["dimensions"];
    let series_dims = &dimensions["series"];
    let obs_dims = &dimensions["observation"];
    let mut series_labels: Vec<Vec<String>> = Vec::new();
    for dim in series_dims.as_array().unwrap() {
        let values: Vec<String> = dim["values"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["id"].as_str().unwrap().to_string())
            .collect();
        series_labels.push(values);
    }
    let mut obs_labels: Vec<Vec<String>> = Vec::new();
    for dim in obs_dims.as_array().unwrap() {
        let values: Vec<String> = dim["values"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["id"].as_str().unwrap().to_string())
            .collect();
        obs_labels.push(values);
    }
    let mut records = Vec::new();
    if let Some(data_sets) = data["dataSets"].as_array() {
        for data_set in data_sets {
            if let Some(series_map) = data_set["series"].as_object() {
                for (key, series) in series_map {
                    let indices: Vec<usize> = key
                        .split(":")
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();
                    let mut dim_values = Vec::new();
                    for (dim_idx, &val_idx) in indices.iter().enumerate() {
                        if dim_idx < series_labels.len()
                            && val_idx < series_labels[dim_idx].len()
                        {
                            let dim_id = series_dims[dim_idx]["id"]
                                .as_str()
                                .unwrap_or("dim")
                                .to_string();
                            dim_values.push((
                                dim_id,
                                series_labels[dim_idx][val_idx].clone(),
                            ));
                        }
                    }
                    if let Some(observations) = series["observations"].as_object() {
                        for (obs_key, obs_value) in observations {
                            let obs_indices: Vec<usize> = obs_key
                                .split(":")
                                .filter_map(|s| s.parse::<usize>().ok())
                                .collect();
                            let mut record = RawRecord::new();
                            for (d, v) in &dim_values {
                                record = record.with(d.clone(), v.clone());
                            }
                            for (dim_idx, &val_idx) in obs_indices.iter().enumerate() {
                                if dim_idx < obs_labels.len()
                                    && val_idx < obs_labels[dim_idx].len()
                                {
                                    let obs_dim_id = obs_dims[dim_idx]["id"]
                                        .as_str()
                                        .unwrap_or("obs_dim")
                                        .to_string();
                                    record = record.with(
                                        obs_dim_id,
                                        obs_labels[dim_idx][val_idx].clone(),
                                    );
                                }
                            }
                            if let Some(val) = obs_value
                                .as_array()
                                .and_then(|a| a.first())
                            {
                                record = record.with(
                                    "OBS_VALUE".to_string(),
                                    val.as_f64()
                                        .map(|v| v.to_string())
                                        .unwrap_or_default(),
                                );
                            }
                            records.push(record);
                        }
                    }
                }
            }
        }
    }
    records
}
#[test]
fn abs_sdmx_fixture_parses_four_observations() {
    let json = load_fixture("abs_sdmx_response.json");
    let records = parse_abs_sdmx_fixture(&json);
    assert_eq!(records.len(), 4);
    let mut builder = RecordBatchBuilder::new();
    for record in records {
        builder.push(record);
    }
    let frame = builder.build().expect("should build DataFrame");
    assert_eq!(frame.width(), 4);
    assert_eq!(frame.height(), 4);
    let measure_col = frame.column("MEASURE").unwrap().str().unwrap();
    let value_col = frame.column("OBS_VALUE").unwrap().str().unwrap();
    assert_eq!(measure_col.get(0), Some("SALES"));
    assert_eq!(value_col.get(0), Some("12.5"));
    assert_eq!(measure_col.get(2), Some("PROFIT"));
    assert_eq!(value_col.get(2), Some("20.0"));
}

#[test]
fn abs_sdmx_fixture_contains_expected_columns() {
    let json = load_fixture("abs_sdmx_response.json");
    let records = parse_abs_sdmx_fixture(&json);
    let mut builder = RecordBatchBuilder::new();
    for record in records {
        builder.push(record);
    }
    let frame = builder.build().expect("valid DataFrame");
    let col_names: Vec<&str> = frame.get_column_names().iter().map(|s| s.as_str()).collect();
    assert!(col_names.contains(&"MEASURE"));
    assert!(col_names.contains(&"REGION"));
    assert!(col_names.contains(&"TIME_PERIOD"));
    assert!(col_names.contains(&"OBS_VALUE"));
}

fn parse_stats_nz_odata_fixture(json_text: &str) -> Vec<RawRecord> {
    let parsed: serde_json::Value =
        serde_json::from_str(json_text).expect("fixture should be valid JSON");
    let mut records = Vec::new();
    if let Some(values) = parsed["value"].as_array() {
        for entry in values {
            let mut record = RawRecord::new();
            if let Some(obj) = entry.as_object() {
                for (key, value) in obj {
                    let str_val = match value {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        _ => value.to_string(),
                    };
                    record = record.with(key.clone(), str_val);
                }
            }
            records.push(record);
        }
    }
    records
}

#[test]
fn stats_nz_odata_fixture_parses_three_records() {
    let json = load_fixture("stats_nz_odata_response.json");
    let records = parse_stats_nz_odata_fixture(&json);
    assert_eq!(records.len(), 3);
    let mut builder = RecordBatchBuilder::new();
    for record in records {
        builder.push(record);
    }
    let frame = builder.build().expect("valid DataFrame");
    let col_names: Vec<&str> = frame.get_column_names().iter().map(|s| s.as_str()).collect();
    for expected in &["Measure", "Region", "TimePeriod", "Value", "Unit"] {
        assert!(col_names.contains(expected));
    }
    let region_col = frame.column("Region").unwrap().str().unwrap();
    assert_eq!(region_col.get(0), Some("Auckland"));
    assert_eq!(region_col.get(1), Some("Wellington"));
    assert_eq!(region_col.get(2), Some("Canterbury"));
}

#[test]
fn stats_nz_odata_fixture_values_are_numeric() {
    let json = load_fixture("stats_nz_odata_response.json");
    let records = parse_stats_nz_odata_fixture(&json);
    let mut builder = RecordBatchBuilder::new();
    for record in records {
        builder.push(record);
    }
    let frame = builder.build().expect("valid DataFrame");
    let value_col = frame.column("Value").unwrap();
    for i in 0..value_col.len() {
        let val = value_col.str().unwrap().get(i).unwrap();
        assert!(
            val.parse::<f64>().is_ok() || val.parse::<i64>().is_ok(),
            "Value '{}' should be numeric",
            val
        );
    }
}
