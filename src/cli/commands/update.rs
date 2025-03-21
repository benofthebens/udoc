//! This module is for the implementation for the update command

use std::cmp::Ordering;
use crate::cli::utils::Paths;
use crate::{config, log};
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use crate::log::exchange::Exchange;
/// >This function assumes that you are already inside an udoc repository checks who you are
/// checks if there are any new images in the image directory and updates the log.md accordingly.
/// ## Todo
/// - TODO: Ensure that when an image is gone from the images dir it is deleted (user-story #8)
/// - TODO: Video update implementation
/// ## Example
/// ```
/// // Cmd: udoc update
/// // images: img.png
/// // config.name: ben
/// // config.email: email@email.com
/// assert_eq!(Ok(()), update()); // log.md is updated
/// ```
/// ## Panics
/// - Panics if the current directory is _not_ a repo
/// - Panics if no _email or name_ is given in the config file
pub fn update() -> io::Result<()> {
    let repo_config = config::read_config(&Paths::Config.get());

    if !Path::new(&Paths::Data.get()).exists() {
        panic!("This path is not a udoc repository");
    } 
    else if repo_config.user.email == String::new() || repo_config.user.username == String::new() {
        panic!("Please choose give your name and email");
    }

    let log_file_metadata= std::fs::metadata(&Paths::Log.get())?;
    let exchange_file_metadata= std::fs::metadata(&Paths::Exchange.get())?;

    let log_file_last_edited = log_file_metadata.modified()?;
    let exchange_file_last_edited = exchange_file_metadata.modified()?;

    match log_file_last_edited.cmp(&exchange_file_last_edited) {
        Ordering::Equal | Ordering::Less => update_log_file(),
        Ordering::Greater => update_exchange_file()
    }?;
    Ok(())
}
fn update_exchange_file() -> io::Result<()> {
    let exchange: Exchange = log::read_log_file(&Paths::Log.get())?;
    std::fs::remove_file(format!("{}/exchange.xml", Paths::Exchange.get()));
    log::exchange::create_exchange_file(&Paths::Exchange.get() ,&exchange)?;
    println!("Updated exchange file");
    Ok(())
}
fn update_log_file() -> io::Result<()> {
    let exchange: Exchange = log::exchange::read_exchange_file(&Paths::Exchange.get())?;
    std::fs::remove_file(Paths::Log.get())?;
    println!("Updated log file");
    log::write_log_file(&exchange, &Paths::Root.get())
}

#[cfg(test)]
mod tests {}
