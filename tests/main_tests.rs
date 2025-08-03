use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_chaf_filters_output() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "error: something failed").unwrap();
    writeln!(file, "info: all good").unwrap();
    writeln!(file, "debug: internal trace").unwrap();

    let mut cmd = Command::cargo_bin("chaf").unwrap();
    cmd.arg("debug | error")
        .arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("info: all good"))
        .stdout(predicate::str::contains("error").not())
        .stdout(predicate::str::contains("debug").not());
}

#[test]
fn test_chaf_with_invert_flag() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "error").unwrap();
    writeln!(file, "info").unwrap();
    writeln!(file, "debug").unwrap();

    let mut cmd = Command::cargo_bin("chaf").unwrap();
    cmd.args(["-i", "debug"])
        .arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("debug"))
        .stdout(predicate::str::contains("error").not())
        .stdout(predicate::str::contains("info").not());
}

#[test]
fn test_chaf_with_report_flag() {
    let mut file = NamedTempFile::new().unwrap();
    writeln!(file, "warn: 1").unwrap();
    writeln!(file, "warn: 2").unwrap();
    writeln!(file, "ok").unwrap();

    let mut cmd = Command::cargo_bin("chaf").unwrap();
    cmd.args(["-r", "warn"])
        .arg(file.path());

    cmd.assert()
        .success()
        .stdout(predicate::str::is_empty()) // 出力されない（--report指定時）
        .stderr(predicate::str::contains("Processed lines: 3"))
        .stderr(predicate::str::contains("Excluded lines: 2"))
        .stderr(predicate::str::contains("Output lines: 1"));
}
