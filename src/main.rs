mod cli;
mod config;
mod log;

use crate::cli::Cmd;
use clap::Parser;

fn main() {
    let cmd = Cmd::parse();
    cmd.get_command().execute();
}
