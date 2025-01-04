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
    ).expect("Unable create config");

    Ok(())
}

/*#[cfg(test)]
mod tests {

    use super::*;
    fn init() -> () {
        let root_path = "./test";
        if Path::new(&root_path).exists() {
            return ();
        }
        std::fs::create_dir("./test");
        std::fs::create_dir("./test/.udoc");

        return ();
    }
    fn remove_files() -> (){
        std::fs::remove_dir_all("./test");
        ();

    }
    #[test]
    fn read_config_test() -> Result<()> {
        init();
        create_config(&"./test".to_string());

        let config: Config = read_config(
            "./test/.udoc/config.json".to_string()
        );

        assert_eq!(config.version, 1);
        assert_eq!(config.log_file_name, "log.md".to_string());
        assert_eq!(config.images_dir, "images".to_string());
        assert_eq!(config.videos_dir, "videos".to_string());

        remove_files();
        Ok(())
    }

    #[test]
    fn create_config_test() -> Result<()> {
        init();

        create_config(&"./test/.udoc".to_string());

        assert!(Path::new("./test/.udoc/config.json").exists());

        Ok(())
    }
}
*/
