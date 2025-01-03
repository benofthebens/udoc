use std::io;
use std::path::Path;
use crate::config;
use crate::config::Config;

pub fn set_config_email(email: &String) -> io::Result<()> {
	let binding = std::env::current_dir()?;
	let root_path = binding.to_str().unwrap();

	let data_path = format!("{root_path}/.udoc");
	let config_path = format!("{data_path}/config.json");

	if !Path::new(&data_path).exists() {
		panic!("This path is not a udoc repository");
	}

	let mut config_json: Config = config::read_config(&config_path);

	config_json.user.email = email.to_string();

	config::create_config(&config_path, config_json).expect("TODO: panic message");

	Ok(())
}