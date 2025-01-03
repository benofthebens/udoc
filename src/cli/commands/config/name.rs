use std::io;
use std::path::Path;
use crate::config;
use crate::config::Config;

pub fn set_config_name(name: &String) -> io::Result<()> {
	if !Path::new("./.udoc").exists() {
		panic!("This path is not a udoc repository");
	}

	let mut config_json: Config = config::read_config("./.udoc/config.json".to_string());
	config_json.user.username = name.to_string();
	std::fs::remove_file("./.udoc/config.json")?;
	config::create_config(&"./.udoc".to_string(), config_json).expect("TODO: panic message");

	Ok(())
}