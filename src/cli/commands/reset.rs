//! This module is the implementation for the reset command
use crate::cli::utils::Paths;
use crate::config;
use crate::config::{Config, User};
use std::path::Path;
use std::{fs, io};
/// > This function removes the directory and all it's contents
/// recreates the directory and config
///
/// # Panics
/// - If the current directory is not an udoc repository
pub fn reset() -> io::Result<()> {
    if !Path::new(&Paths::Data.get()).exists() {
        panic!("This is not a udoc repository ");
    }

    fs::remove_dir_all(&Paths::Data.get())?;
    fs::create_dir(&Paths::Data.get())?;

    config::create_config(
        &Paths::Config.get(),
        Config::new(
            1,
            "log.md".to_string(),
            "images".to_string(),
            "videos".to_string(),
            User {
                username: String::new(),
                email: String::new(),
            },
        ),
    )?;

    Ok(())
}
