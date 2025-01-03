use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use crate::{config, log};

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