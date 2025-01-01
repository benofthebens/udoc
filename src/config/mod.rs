use std::io;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde_json::{Result, Value};

#[derive(Deserialize, Debug)]
pub struct Config {
    version: u8,
    log_file_name: String, 
    pub images_dir: String, 
    pub videos_dir: String, 
}

pub fn read_config(full_path: String) -> Config {
    let config_str: String = fs::read_to_string(full_path)
        .expect("Unable to read file");
    let config: Config = serde_json::from_str(&config_str)
        .expect("Unable to convert to Config struct");
    println!("{:?}", &config); 
    config
}

pub fn create_config(file_path: &String) -> Result<()> {

    let default_config = r#"{
    "version": 1,
    "log_file_name": "log.md",
    "images_dir": "images",
    "videos_dir": "videos"
}"#;

    fs::write(format!("{file_path}/config.json"), default_config);

    Ok(())
}

#[cfg(test)]
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

