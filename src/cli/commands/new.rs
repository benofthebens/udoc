//! This module initialises an udoc repository creating the directories and files
use crate::config::{Config, User};
use crate::log::read_log_file;
use crate::log::exchange::{create_exchange_file, Exchange};
use crate::log::section::Section;
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

    // Gets the environment directory
    let environment_path= std::env::current_dir()?;
    let environment_path = environment_path.to_str().unwrap();

    let date = Local::now().date_naive();
    let error_name = format!("{name}-{date}");

    let root_path: String = format!("{environment_path}/{error_name}");
    let data_path: String = format!("{root_path}/.udoc");
    let config_path: String = format!("{root_path}/.udoc/config.json");
    let image_path: String = format!("{root_path}/{image_dir_name}");
    let video_path: String = format!("{root_path}/{video_dir_name}");

    // Creates the directory
    fs::create_dir(&root_path)?;
    fs::create_dir(&data_path)?;
    fs::create_dir(&image_path)?;
    fs::create_dir(&video_path)?;

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
    )?;

    // Creates the files
    let exchange = Exchange {
        title: error_name,
        section: vec![
            Section {
                heading: String::from("Description"),
                text: description.to_string(), 
                images: None
            }
        ]
    };

    let xml: String = create_exchange_file(
        &format!("{root_path}/.udoc/exchange/"),
        &exchange
    )?;

    println!("{}", xml);

    log::write_log_file(&exchange, &root_path)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    pub fn new_test() -> () {

    }
}