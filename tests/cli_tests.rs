use chaf::cli::CliOptions;
use std::path::PathBuf;
use clap::Parser;

#[test]
fn test_parse_minimum_args() {
    let args = ["chaf", "foo"];
    let opts = CliOptions::parse_from(&args);
    assert_eq!(opts.query, "foo");
    assert_eq!(opts.input_file, None);
    assert!(!opts.invert);
    assert!(!opts.report);
}

#[test]
fn test_parse_with_file() {
    let args = ["chaf", "ERROR", "log.txt"];
    let opts = CliOptions::parse_from(&args);
    assert_eq!(opts.query, "ERROR");
    assert_eq!(opts.input_file, Some(PathBuf::from("log.txt")));
}

#[test]
fn test_parse_with_flags() {
    let args = ["chaf", "-i", "-r", "query", "file.log"];
    let opts = CliOptions::parse_from(&args);
    assert!(opts.invert);
    assert!(opts.report);
    assert_eq!(opts.query, "query");
    assert_eq!(opts.input_file, Some(PathBuf::from("file.log")));
}

#[test]
fn test_parse_long_flags() {
    let args = ["chaf", "--invert", "--report", "x & y"];
    let opts = CliOptions::parse_from(&args);
    assert!(opts.invert);
    assert!(opts.report);
    assert_eq!(opts.query, "x & y");
    assert_eq!(opts.input_file, None);
}

#[test]
fn test_missing_query_should_fail() {
    let args = ["chaf"];
    let result = CliOptions::try_parse_from(&args);
    assert!(result.is_err());
}
