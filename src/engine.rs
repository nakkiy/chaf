use std::io::{BufRead, Write};
use tracing::{debug, error, info};

/// テキストストリームを読み取り、フィルタ関数で処理し、出力に書き出す。
///
/// - `reader` : 入力ソース（ファイル or stdin）
/// - `writer` : 出力先（stdout）
/// - `filter` : 行ごとのフィルタ関数。trueなら除去対象（invert=false時）
/// - `invert` : 除去ではなく抽出モードに切り替える（=grep的な動作）
/// - `report` : 処理件数などをstderrに出力するかどうか
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

        // CRLF対応: \r\n を \n に変換
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
                debug!("Line {}: matched={} → output={}", total, matched, matched);
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
