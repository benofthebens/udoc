//! This module is to map the subcommands enums to their corresponding functions
mod config;
mod new;
mod reset;
mod update;
use crate::cli::commands::config::ConfigCommands;
use crate::cli::commands::new::new;
use crate::cli::commands::reset::reset;
use crate::cli::commands::update::update;
use chrono::prelude::*;
use clap::Subcommand;
use std::io;

/// > Represents the different subcommand variants with their corresponding arguments
///
/// ## Example
/// ```
/// - udoc new --name <name-of-error>
///            --description <description-of-error>
///            --image_dir_name <image-directory>
///            --video_dir_name <video-directory>
///            --error_number <error_number>
/// - udoc update
/// - udoc config <config-subcommand>
/// - udoc reset
/// ```
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
    /// Maps the command to corresponding function
    ///
    /// ## Example
    /// ```
    /// // Cmd: `udoc new -n err`
    /// // Maps to: `New` enum variant
    /// // executes: `new("err", "", "", 1)`
    /// ```
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
