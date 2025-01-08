mod cli;
mod config;
mod log;

use crate::cli::Cli;
use clap::Parser;
use crate::log::{read_log_file};

fn main() {
    // let cmd = Cli::parse();
    // cmd.get_command().execute();
    let log_path = "./log.md".to_string();
    let contents= match read_log_file(&log_path) {
        Ok(contents) => contents,
        _ => panic!("AHHH")
    };
    println!("{:?}", contents);
}
