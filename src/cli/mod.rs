mod commands;

use crate::cli::commands::Commands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}
impl Cli {
    pub fn get_command(&self) -> &Commands {
        &self.command
    }
}
