mod email;
mod name;

use crate::cli::commands::config::email::set_config_email;
use crate::cli::commands::config::name::set_config_name;
use clap::Subcommand;
use std::io;

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
