use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "chaf",
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None
)]
pub struct CliOptions {
    #[arg(name = "QUERY")]
    pub query: String,

    #[arg(name = "FILE")]
    pub input_file: Option<PathBuf>,

    #[arg(short, long)]
    pub report: bool,

    #[arg(short, long)]
    pub invert: bool,
}

pub fn parse_args() -> CliOptions {
    CliOptions::parse()
}
