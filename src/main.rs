mod cli;
mod config;
mod log;

use crate::cli::Cli;
use clap::Parser;

fn main() {
    let cmd = Cli::parse();
    cmd.get_command().execute();
}
