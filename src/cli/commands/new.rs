//! This module initialises an udoc repository creating the directories and files
use crate::config::{Config, User};
use crate::log::read_log_file;
use crate::log::exchange::{create_exchange_file, Exchange};
use crate::{config, log};

use chrono::Local;
use std::{fs, io};
/// This takes in the arguments from the cli finds the current directory
/// creates a name which includes the __name__ parameter and the current date
/// initialises all paths for a common udoc repo.
///
/// ## Example
/// ```
/// /*
///  * To create a udoc repository with:
///  * name: 'err-CURRENT-DATE'
///  * description: 'This is a description'
///  * image_dir_name: 'images'
///  * video_dir_name: 'videos'
///  */
/// new("err","This is a description", "images", "videos");
/// ```
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
    fs::create_dir(&image_path).expect("TODO: panic message");
    fs::create_dir(&video_path).expect("TODO: panic message");

    let config: Config = Config::new(
        1,
        "log.md".to_string(),
        image_dir_name.to_string(),
        video_dir_name.to_string(),
        User {
            username: String::new(),
            email: String::new(),
        },
    );

    config::create_config(
        &config_path,
        config.clone(),
    )
    .expect("TODO: panic message");

    // Creates the files
    log::create_log_file(&root_path, &error_name, description, config).expect("TODO: panic message");
    let exchange: Exchange = log::read_log_file(&format!("{root_path}/log.md"))?;
    log::exchange::create_exchange_file(&format!("{root_path}/.udoc/exchange/"), exchange);
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

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

        new(
            &test_name,
            &String::new(),
            &"".to_string(),
            &"".to_string(),
            &0,
        )
        .expect("TODO: panic message");

        assert!(Path::new(&full_path).exists());
        assert!(Path::new(&udoc_path).exists());
        assert!(Path::new(&images_path).exists());
        assert!(Path::new(&videos_path).exists());
        assert!(Path::new(&log_file).exists());

        destroy(format!("{test_name}-{date}"));

        Ok(())
    }
}
