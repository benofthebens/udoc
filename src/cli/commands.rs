mod config;
mod new;
mod reset;
mod update;

use crate::cli::commands::config::ConfigCommands;
use crate::cli::commands::new::new;
use crate::cli::commands::update::update;
use crate::cli::commands::reset::reset;
use chrono::prelude::*;
use clap::Subcommand;
use std::io;

#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        #[arg(short, long)]
        name: String,

        #[arg(short, long, default_value_t = String::new())]
        description: String,

        #[arg(short, long, default_value_t = String::from("images"))]
        image_dir_name: String,

        #[arg(short, long, default_value_t = String::from("videos"))]
        video_dir_name: String,

        #[arg(short, long, default_value_t = 1)]
        error_number: u8,
    },
    Update,
    Config {
        #[command(subcommand)]
        cmd: ConfigCommands,
    },
    Reset,
}
impl Commands {
    pub fn execute(&self) -> io::Result<()> {
        match self {
            Commands::New {
                name,
                description,
                image_dir_name,
                video_dir_name,
                error_number,
            } => new(
                name,
                description,
                image_dir_name,
                video_dir_name,
                error_number,
            ),
            Commands::Update => update(),
            Commands::Config { cmd } => cmd.execute(),
            Commands::Reset => reset(),
        }
    } 
}