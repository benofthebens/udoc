use std::{fs, io};
use std::path::Path;
use crate::config;
use crate::config::{Config, User};

pub fn reset() -> io::Result<()> {
	if !Path::new("./.udoc").exists() {
		panic!("This is not a udoc repository ");
	}
	fs::remove_dir_all("./.udoc").expect("TODO: panic message");
	fs::create_dir("./.udoc");

	config::create_config(
		&"./.udoc".to_string(),
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