mod commands;

use crate::cmd::commands::Commands;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cmd {
    #[command(subcommand)]
    command: Commands,
}
impl Cmd {
    pub fn get_command(&self) -> &Commands {
        &self.command
    }
}
