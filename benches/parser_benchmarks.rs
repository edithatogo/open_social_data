use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Re-use the fixture-loading and parser functions from the regression tests.
// In a real benchmark setup these would live in a shared module.
use std::fs;
use std::path::Path;

fn load_fixture(name: &str) -> String {
    let path = Path::new("tests").join("fixtures").join(name);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("could not load fixture `{}`", path.display()))
}

fn parse_abs_sdmx(json_text: &str) -> usize {
    let parsed: serde_json::Value =
        serde_json::from_str(json_text).expect("valid JSON");
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
    let mut count = 0usize;
    if let Some(data_sets) = data["dataSets"].as_array() {
        for data_set in data_sets {
            if let Some(series_map) = data_set["series"].as_object() {
                for (_key, series) in series_map {
                    if let Some(observations) = series["observations"].as_object() {
                        count += observations.len();
                    }
                }
            }
        }
    }
    count
}

fn parse_stats_nz_odata(json_text: &str) -> usize {
    let parsed: serde_json::Value =
        serde_json::from_str(json_text).expect("valid JSON");
    if let Some(values) = parsed["value"].as_array() {
        values.len()
    } else {
        0
    }
}

fn bench_abs_sdmx_parse(c: &mut Criterion) {
    let json = load_fixture("abs_sdmx_response.json");
    c.bench_function("abs_sdmx_parse", |b| {
        b.iter(|| parse_abs_sdmx(black_box(&json)))
    });
}

fn bench_stats_nz_odata_parse(c: &mut Criterion) {
    let json = load_fixture("stats_nz_odata_response.json");
    c.bench_function("stats_nz_odata_parse", |b| {
        b.iter(|| parse_stats_nz_odata(black_box(&json)))
    });
}

criterion_group!(benches, bench_abs_sdmx_parse, bench_stats_nz_odata_parse);
criterion_main!(benches);
