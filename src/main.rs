mod cli;
mod config;
mod log;

use crate::cli::Cli;
use clap::Parser;
use crate::log::{create_exchange_file, read_log_file};

fn main() {
    // let cmd = Cli::parse();
    // cmd.get_command().execute();
    let log_path = "./log.md".to_string();
    let contents= match read_log_file(&log_path) {
        Ok(contents) => contents,
        _ => panic!("AHHH")
    };
    create_exchange_file(contents);
}
