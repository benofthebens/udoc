mod commands;

use clap::Parser;
use crate::cmd::commands::Commands;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cmd {
    #[command(subcommand)]
    command: Commands
}
