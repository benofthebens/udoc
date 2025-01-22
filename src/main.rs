mod cli; mod config;
mod log;

use crate::cli::Cli;
use clap::Parser;
use crate::log::{read_log_file};

fn main() {
    let cmd = Cli::parse();
    cmd.get_command().execute();
}
