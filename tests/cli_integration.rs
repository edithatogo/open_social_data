/// Integration tests for the `open-social-data-cli` binary.
///
/// These tests run the compiled binary as a subprocess to verify
/// that CLI argument parsing, subcommand routing, and output
/// formatting work correctly end-to-end.
///
/// Note: Cargo automatically sets `CARGO_BIN_EXE_open_social_data_cli`
/// to the path of the compiled binary when running integration tests.
use std::path::PathBuf;
use std::process::Command;

/// Path to the compiled CLI binary (set by Cargo for the hyphenated bin target).
fn cli_binary() -> PathBuf {
    std::env::var_os("CARGO_BIN_EXE_open-social-data-cli")
        .or_else(|| std::env::var_os("CARGO_BIN_EXE_open_social_data_cli"))
        .map(PathBuf::from)
        .expect("CARGO_BIN_EXE_open-social-data-cli should be set by cargo")
}

#[test]
fn cli_help_prints_without_error() {
    let output = Command::new(cli_binary())
        .arg("--help")
        .output()
        .expect("failed to run CLI with --help");
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // clap prints usage information to stdout
    assert!(
        stdout.contains("open-social-data-cli"),
        "expected binary name in help output, got: {stdout}{stderr}"
    );
    assert!(
        stdout.contains("fetch"),
        "expected 'fetch' in help output, got: {stdout}{stderr}"
    );
    assert!(
        stdout.contains("list"),
        "expected 'list' in help output, got: {stdout}{stderr}"
    );
    assert!(
        stdout.contains("status"),
        "expected 'status' in help output, got: {stdout}{stderr}"
    );
    assert!(
        stdout.contains("catalog"),
        "expected 'catalog' in help output, got: {stdout}{stderr}"
    );
    assert!(
        stdout.contains("validate"),
        "expected 'validate' in help output, got: {stdout}{stderr}"
    );
    assert!(
        stdout.contains("examples"),
        "expected 'examples' in help output, got: {stdout}{stderr}"
    );
}

#[test]
fn cli_help_subcommand_prints_without_error() {
    let output = Command::new(cli_binary())
        .arg("catalog")
        .arg("--help")
        .output()
        .expect("failed to run CLI with catalog --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("catalog"),
        "expected 'catalog' in help output, got: {stdout}"
    );
    assert!(
        stdout.contains("list"),
        "expected 'list' in catalog help output, got: {stdout}"
    );
    assert!(
        stdout.contains("search"),
        "expected 'search' in catalog help output, got: {stdout}"
    );
    assert!(
        stdout.contains("sync"),
        "expected 'sync' in catalog help output, got: {stdout}"
    );
}

#[test]
fn cli_validate_help_prints_without_error() {
    let output = Command::new(cli_binary())
        .arg("validate")
        .arg("--help")
        .output()
        .expect("failed to run CLI with validate --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("dataset-packs"),
        "expected 'dataset-packs' in validate help output, got: {stdout}"
    );
    assert!(
        stdout.contains("source-metadata"),
        "expected 'source-metadata' in validate help output, got: {stdout}"
    );
    assert!(
        stdout.contains("medium-term"),
        "expected 'medium-term' in validate help output, got: {stdout}"
    );
}

#[test]
fn cli_examples_help_prints_without_error() {
    let output = Command::new(cli_binary())
        .arg("examples")
        .arg("--help")
        .output()
        .expect("failed to run CLI with examples --help");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("myhospitals-summary"),
        "expected 'myhospitals-summary' in examples help output, got: {stdout}"
    );
    assert!(
        stdout.contains("source-metadata-inventory"),
        "expected 'source-metadata-inventory' in examples help output, got: {stdout}"
    );
}

#[test]
fn cli_version_prints_without_error() {
    let output = Command::new(cli_binary())
        .arg("--version")
        .output()
        .expect("failed to run CLI with --version");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.is_empty(), "expected non-empty version output");
}

#[test]
fn cli_invalid_subcommand_returns_error() {
    let output = Command::new(cli_binary())
        .arg("nonexistent-subcommand")
        .output()
        .expect("failed to run CLI with invalid subcommand");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("error") || stderr.contains("unrecognized"),
        "expected error message for invalid subcommand, got: {stderr}"
    );
}

#[test]
fn cli_fetch_without_required_args_returns_error() {
    let output = Command::new(cli_binary())
        .arg("fetch")
        .output()
        .expect("failed to run CLI with fetch without args");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("error") || stderr.contains("required"),
        "expected error for missing fetch args, got: {stderr}"
    );
}
