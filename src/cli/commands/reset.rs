use std::{fs, io};
use std::path::Path;
use crate::config;
use crate::config::{Config, User};

pub fn reset() -> io::Result<()> {
	let binding = std::env::current_dir()?;
	let root_path = binding.to_str().unwrap();

	let data_path = format!("{root_path}/.udoc");
	let config_path = format!("{data_path}/config.json");

	if !Path::new(&data_path).exists() {
		panic!("This is not a udoc repository ");
	}

	fs::remove_dir_all(&data_path).expect("TODO: panic message");
	fs::create_dir(&data_path).expect("Unable to create directory");

	config::create_config(
		&config_path,
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
	)
	.expect("TODO: panic message");

	Ok(())
}