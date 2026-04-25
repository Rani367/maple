use assert_cmd::prelude::*;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

fn fixture_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn maple_command() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("maple"))
}

fn scan_fixture_json(name: &str) -> Value {
    let assert = maple_command()
        .current_dir(fixture_path(name))
        .args(["scan", ".", "--json"])
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();

    serde_json::from_str(&stdout).unwrap()
}

fn language_files(report: &Value, language: &str) -> u64 {
    report["languages"][language]["files"].as_u64().unwrap_or(0)
}

fn normalize_json_root(mut report: Value) -> Value {
    report["root"] = Value::String("<ROOT>".to_owned());
    report
}

#[test]
fn scan_json_for_tiny_rust_fixture() {
    let report = scan_fixture_json("tiny-rust-crate");

    assert_eq!(report["files"], 4);
    assert_eq!(language_files(&report, "Rust"), 2);
    assert_eq!(language_files(&report, "TOML"), 1);
    assert_eq!(language_files(&report, "Markdown"), 1);
}

#[test]
fn scan_json_for_tiny_typescript_fixture() {
    let report = scan_fixture_json("tiny-typescript-app");

    assert_eq!(report["files"], 5);
    assert_eq!(language_files(&report, "TypeScript"), 2);
    assert_eq!(language_files(&report, "JSON"), 2);
    assert_eq!(language_files(&report, "Markdown"), 1);
}

#[test]
fn scan_json_for_mixed_fixture() {
    let report = scan_fixture_json("mixed-repo-with-docs-config");

    assert_eq!(report["files"], 8);
    assert_eq!(language_files(&report, "Markdown"), 2);
    assert_eq!(language_files(&report, "Other"), 2);
    assert_eq!(language_files(&report, "JSON"), 1);
    assert_eq!(language_files(&report, "Shell"), 1);
    assert_eq!(language_files(&report, "TOML"), 1);
    assert_eq!(language_files(&report, "TypeScript"), 1);
}

#[test]
fn scan_json_snapshot_normalizes_root() {
    let report = normalize_json_root(scan_fixture_json("mixed-repo-with-docs-config"));

    insta::assert_json_snapshot!("mixed_repo_json", report);
}

#[test]
fn scan_terminal_snapshot_normalizes_root() {
    let fixture = fixture_path("tiny-rust-crate");
    let root = fixture.canonicalize().unwrap().display().to_string();
    let assert = maple_command()
        .current_dir(&fixture)
        .args(["scan", "."])
        .assert()
        .success();
    let stdout = String::from_utf8(assert.get_output().stdout.clone()).unwrap();
    let normalized = stdout.replace(&root, "<ROOT>");

    insta::assert_snapshot!("tiny_rust_terminal", normalized);
}

#[test]
fn invalid_path_reports_useful_error() {
    let missing = fixture_path("does-not-exist");
    let output = maple_command()
        .args(["scan", missing.to_str().unwrap(), "--json"])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("failed to scan"));
    assert!(stderr.contains("failed to resolve"));
}
