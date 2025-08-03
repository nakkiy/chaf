use chaf::engine::run_filter;
use std::io::{BufReader, Cursor};

fn make_reader(lines: &[&str]) -> Box<dyn std::io::BufRead> {
    let content = lines.join("\n");
    Box::new(BufReader::new(Cursor::new(content)))
}

#[test]
fn test_basic_output_logic() {
    let input = make_reader(&["foo", "bar", "baz"]);
    let mut output = Vec::new();

    let filter = |line: &[u8]| Ok(!line.contains(&b'b')); // b を含まない → 出力

    run_filter(input, &mut output, filter, false).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "foo\n");
}

#[test]
fn test_crlf_conversion() {
    let crlf_input = "line1\r\nline2\r\nline3\r\n";
    let input = Box::new(BufReader::new(Cursor::new(crlf_input)));
    let mut output = Vec::new();

    let filter = |_line: &[u8]| Ok(true); // 全行出力

    run_filter(input, &mut output, filter, false).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "line1\nline2\nline3\n");
}

#[test]
fn test_filter_error_outputs_line() {
    let input = make_reader(&["ok line", "bad line"]);
    let mut output = Vec::new();

    let filter = |line: &[u8]| {
        let text = std::str::from_utf8(line)?;
        if text.contains("bad") {
            Err(anyhow::anyhow!("mock error"))
        } else {
            Ok(true)
        }
    };

    run_filter(input, &mut output, filter, false).unwrap();

    let result = String::from_utf8(output).unwrap();
    assert_eq!(result, "ok line\nbad line");
}

#[test]
fn test_report_mode_no_output() {
    let input = make_reader(&["a", "b", "c"]);
    let mut output = Vec::new();

    let filter = |_line: &[u8]| Ok(true);

    run_filter(input, &mut output, filter, true).unwrap();

    assert_eq!(output.len(), 0); // --report のときは出力抑制
}
