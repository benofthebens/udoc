//! This module is for the implementation for the update command

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

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(Paths::Log.get())?;
    
    log::update_images(
        &mut file,
        &Paths::Root.get(),
        repo_config
    ).expect("TODO: panic message");

    let exchange: Exchange = log::read_log_file(&Paths::Log.get())?;
    std::fs::remove_file(format!("{}/exchange.xml",Paths::Exchange.get()));
    log::exchange::create_exchange_file(&Paths::Exchange.get() ,exchange);
    Ok(())
}

#[cfg(test)]
mod tests {}
