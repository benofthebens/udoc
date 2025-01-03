mod name;
mod email;

use std::io;
use crate::cli::commands::config::name::set_config_name;
use crate::cli::commands::config::email::set_config_email;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Name {
        #[arg(short, long)]
        name: String,
    },
    Email {
        #[arg(short, long)]
        email: String,
    },
}
impl ConfigCommands {
    pub fn execute(&self) -> io::Result<()> {
        match self {
            ConfigCommands::Name { name } => set_config_name(name),
            ConfigCommands::Email { email } => set_config_email(email),
        }
    }
}