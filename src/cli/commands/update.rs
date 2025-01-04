//! This module is for the implementation for the update command

use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use crate::{config, log};


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

	let binding = std::env::current_dir()?;
	let root_path: &str = binding.to_str().unwrap();
	let data_path = format!("{root_path}/.udoc");
	let config_path = format!("{data_path}/config.json");
	let log_path = format!("{root_path}/log.md");

	let repo_config = config::read_config(&config_path);

	if !Path::new(&data_path).exists() {
		panic!("This path is not a udoc repository");
	} else if repo_config.user.email == String::new() || repo_config.user.username == String::new() {
		panic!("Please choose give your name and email");
	}

	let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open(log_path)?;

	log::update_images(&mut file, &root_path.to_string())
		.expect("TODO: panic message");

	Ok(())
}