use std::{fs, io};
use chrono::Local;
use crate::{config, log};
use crate::config::{Config, User};

pub fn new(
	name: &String,
	description: &String,
	image_dir_name: &String,
	video_dir_name: &String,
	error_number: &u8,
) -> io::Result<()> {

	// Gets the current directory
	let binding = std::env::current_dir()?;
	let environment_path: &str = binding.to_str().unwrap();

	let date = Local::now().date_naive();
	let error_name = format!("{name}-{date}");

	let root_path: String = format!("{environment_path}/{error_name}");
	let data_path: String = format!("{root_path}/.udoc");
	let config_path: String = format!("{root_path}/.udoc/config.json");
	let image_path: String = format!("{root_path}/{image_dir_name}");
	let video_path: String = format!("{root_path}/{video_dir_name}");

	// Creates the directory
	fs::create_dir(&root_path).expect("TODO: panic message");
	fs::create_dir(&data_path).expect("TODO: panic message");

	config::create_config(
		&config_path,
		Config::new(
			1,
			"log.md".to_string(),
			image_dir_name.to_string(),
			video_dir_name.to_string(),
			User {
				username: String::new(),
				email: String::new(),
			},
		),
	).expect("TODO: panic message");

	fs::create_dir(&image_path).expect("TODO: panic message");
	fs::create_dir(&video_path).expect("TODO: panic message");

	// Creates the files
	log::create_log_file(&root_path, &error_name, description)
		.expect("TODO: panic message");

	Ok(())
}
#[cfg(test)]
mod tests {
	use std::path::Path;
	use super::*;

	fn setup() -> () {}
	fn destroy(name: String) -> () {
		std::fs::remove_dir_all(name);
		()
	}

	#[test]
	fn init_test() -> io::Result<()> {
		let test_name: String = "test".to_string();
		let date = Local::now().date_naive();
		let full_path = format!("./{test_name}-{date}");

		let udoc_path = format!("{}/.udoc", &full_path);
		let images_path = format!("{}/images", &full_path);
		let videos_path = format!("{}/videos", &full_path);
		let log_file = format!("{}/log.md", &full_path);

		new(&test_name, &String::new(), &"".to_string(), &"".to_string(), &0).expect("TODO: panic message");

		assert!(Path::new(&full_path).exists());
		assert!(Path::new(&udoc_path).exists());
		assert!(Path::new(&images_path).exists());
		assert!(Path::new(&videos_path).exists());
		assert!(Path::new(&log_file).exists());

		destroy(format!("{test_name}-{date}"));

		Ok(())
	}
}
