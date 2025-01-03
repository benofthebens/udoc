use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use crate::{config, log};

pub fn update() -> io::Result<()> {
	let binding = std::env::current_dir()?;
	let path: Option<&str> = binding.to_str();
	let path: &str = match path {
		Some(path) => path,
		None => panic!("Path provided is None"),
	};
	let config_path = format!("{path}/.udoc");
	let config_json = config::read_config(format!("{config_path}/config.json"));
	if !Path::new(&config_path).exists() {
		panic!("This path is not a udoc repository");
	} else if config_json.user.email == String::new() || config_json.user.username == String::new() {
		panic!("Please choose give your name and email");
	}

	let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open(format!("{path}/log.md"))?;

	let root_path = path.to_string();

	log::update_images(&mut file, &root_path)
		.expect("TODO: panic message");

	Ok(())
}