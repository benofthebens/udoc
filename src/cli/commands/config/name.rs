use std::io;
use std::path::Path;
use crate::config;
use crate::config::Config;

pub fn set_config_name(name: &String) -> io::Result<()> {

	let binding = std::env::current_dir()?;
	let root_path = binding.to_str().unwrap();

	let data_path = format!("{root_path}/.udoc");
	let config_path = format!("{data_path}/config.json");

	if !Path::new(&data_path).exists() {
		panic!("This path is not a udoc repository");
	}

	let mut config: Config = config::read_config(&config_path);
	config.user.username = name.to_string();
	std::fs::remove_file(&config_path)?;
	config::create_config(&data_path, config).expect("TODO: panic message");

	Ok(())
}