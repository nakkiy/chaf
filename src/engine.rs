use std::io::{BufRead, Write};
use tracing::{error, info, trace};

// Processes a text stream by applying a per-line filter function and writing matching lines to the output.
//   `reader` – The input source (e.g., a file or standard input).
//   `writer` – The output destination (e.g., standard output).
//   `filter` – A function applied to each line. Returns `true` to include the line in the output (when `invert = false`).
//   `invert` – If `true`, reverses the filter logic (i.e., switches to inclusion instead of exclusion; grep-like behavior).
//   `report` – If `true`, suppresses output and prints processing statistics to standard error.
//
// Returns an error if reading from the input or writing to the output fails, or if the filter function itself returns an error.
pub fn run_filter(
    mut reader: Box<dyn BufRead>,
    writer: &mut dyn Write,
    filter: impl Fn(&[u8]) -> Result<bool, anyhow::Error>,
    report: bool,
) -> Result<(), anyhow::Error> {
    let mut buffer = Vec::with_capacity(4096);

    let mut total = 0usize;
    let mut skipped = 0usize;
    let mut written = 0usize;

    loop {
        buffer.clear();
        let n = reader.read_until(b'\n', &mut buffer)?;
        if n == 0 {
            break;
        }
        total += 1;

        // Handle CRLF: convert \r\n to \n
        if buffer.ends_with(b"\r\n") {
            buffer.truncate(buffer.len() - 2);
            buffer.push(b'\n');
        }

        match filter(&buffer) {
            Ok(matched) => {
                if matched {
                    if !report {
                        writer.write_all(&buffer)?;
                    }
                    written += 1;
                } else {
                    skipped += 1;
                }
                trace!("Line {}: matched={} → output={}", total, matched, matched);
            }
            Err(e) => {
                error!("Filter error at line {}: {}", total, e);
                if !report {
                    writer.write_all(&buffer)?;
                }
                written += 1;
            }
        }
    }

    if report {
        info!(
            "Done. Total={}, Skipped={}, Output={}",
            total, skipped, written
        );
        eprintln!(
            "Processed lines: {}\nExcluded lines: {}\nOutput lines: {}",
            total, skipped, written
        );
    }

    Ok(())
}
