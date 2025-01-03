mod cmd;
mod config;
mod log;

use crate::cmd::Cmd;
use clap::Parser;

fn main() {
    let cmd = Cmd::parse();
    cmd.get_command().execute();
}
