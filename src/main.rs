mod cli;
mod core;
mod engine;
mod util;

use crate::cli::parse_args;
use crate::core::filter::build_filter;
use crate::core::parser::parse_query;
use crate::util::init_logging;
use std::fs::File;
use std::io::{self, BufReader};

fn main() {
    init_logging();
    tracing::info!("chaf started");

    let opts = parse_args();

    let ast = match parse_query(&opts.query) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("構文エラー: {e}");
            std::process::exit(1);
        }
    };

    let filter = match build_filter(&ast, opts.invert) {
        Ok(filter) => filter,
        Err(e) => {
            eprintln!("フィルタ構築エラー: {e}");
            std::process::exit(1);
        }
    };

    let reader: Box<dyn io::BufRead> = match &opts.input_file {
        Some(path) => match File::open(path) {
            Ok(file) => Box::new(BufReader::new(file)),
            Err(e) => {
                eprintln!("ファイルオープンエラー: {e}");
                std::process::exit(1);
            }
        },
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut writer = io::stdout();

    if let Err(e) = engine::run_filter(reader, &mut writer, filter, opts.report) {
        eprintln!("実行中エラー: {e}");
        std::process::exit(1);
    }
}
