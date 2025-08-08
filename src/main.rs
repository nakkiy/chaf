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
use tracing::{debug, info};

fn main() {
    init_logging();
    info!("chaf started");

    let opts = parse_args();
    debug!("line options: {:?}", opts);

    let ast = match parse_query(&opts.query) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Syntax error: {e}");
            info!("chaf ended with error");
            std::process::exit(1);
        }
    };
    debug!("Parsed AST: {:?}", ast);

    let filter = match build_filter(&ast, opts.invert) {
        Ok(filter) => filter,
        Err(e) => {
            eprintln!("Filter build error: {e}");
            info!("chaf ended with error");
            std::process::exit(1);
        }
    };

    let reader: Box<dyn io::BufRead> = match &opts.input_file {
        Some(path) => match File::open(path) {
            Ok(file) => Box::new(BufReader::new(file)),
            Err(e) => {
                eprintln!("Failed to open file: {e} at path: {}", path.display());
                info!("chaf ended with error");
                std::process::exit(1);
            }
        },
        None => Box::new(BufReader::new(io::stdin())),
    };

    let mut writer = io::stdout();

    if let Err(e) = engine::run_filter(reader, &mut writer, filter, opts.report) {
        eprintln!("Runtime error: {e}");
        info!("chaf ended with error");
        std::process::exit(1);
    }
    info!("chaf ended successfully");
}
