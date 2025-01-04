use std::fs;
use serde::Serialize;
use serde::Deserialize;
use serde_json::{Result, Value};
pub use crate::config::user::User;
mod user;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub version: u8,
    pub log_file_name: String,
    pub images_dir: String,
    pub videos_dir: String,
    pub user: User,
}
impl Config {
    pub fn new(
        version: u8,
        log_file_name: String,
        images_dir: String,
        videos_dir: String,
        user: User,
    ) -> Self {

        Self {
            version,
            log_file_name,
            images_dir,
            videos_dir,
            user,
        }
    }
}
pub fn read_config(config_path: &String) -> Config {
    let config_str: String = fs::read_to_string(config_path).expect("Unable to read file");
    let config: Config = serde_json::from_str(&config_str).expect("Unable to convert to Config struct");
    config
}

pub fn create_config(config_path: &String, config: Config) -> Result<()> {
    fs::write(
        config_path,
        serde_json::to_string_pretty(&config)?,
    ).expect("Unable create config at {config_path}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
    fn setup(data_path: &String) -> () {
       fs::create_dir(data_path).unwrap();
    }
    fn teardown(config_path: &String) {
        fs::remove_dir_all(config_path).unwrap();
    }
    fn default_config() -> Config {
        Config {
            version: 1,
            log_file_name: "log.md".to_string(),
            images_dir: "images".to_string(),
            videos_dir: "videos".to_string(),
            user: User {
                username: "user".to_string(),
                email: "user@email.com".to_string(),
            }
        }
    }
    #[test]
    fn create_config_test() {
        let env_path = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        let data_path: String = format!("{env_path}/.udoc");
        let config_path: String = format!("{data_path}/config.json");
        setup(&data_path);

        create_config(&config_path, default_config()).unwrap();

        assert!(Path::new(&config_path).exists());

        teardown(&data_path);
    }
    #[test]
    fn read_config_test() {

    }
    #[test]
    fn new_config_test() {
        let config: Config = Config::new(
            1,
            "log.md".to_string(),
            "images".to_string(),
            "videos".to_string(),
            User {
                username: "user".to_string(),
                email: "user@email.com".to_string(),
            }
        );

        assert_eq!(config.version, 1);
        assert_eq!(config.log_file_name, "log.md");
        assert_eq!(config.images_dir, "images");
        assert_eq!(config.videos_dir, "videos");
        assert_eq!(config.user.username, "user");
        assert_eq!(config.user.email, "user@email.com")
    }
}

