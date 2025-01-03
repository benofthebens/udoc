use std::fs::File;
use std::fs;
use std::io;
use std::path::Path;
use clap::Subcommand;
use std::fs::OpenOptions;
use crate::config::Config;
use crate::config;
use crate::log; 
use chrono::prelude::*;
use crate::config::User;

#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        #[arg(short, long)]
        name: String,

        #[arg(short, long, default_value_t = String::new())]
        description: String,

        #[arg(short, long, default_value_t = String::from("images"))]
        image_dir_name: String,

        #[arg(short, long, default_value_t = String::from("videos"))]
        video_dir_name: String,

        #[arg(short, long, default_value_t = 1)]
        error_number: u8
    },
    Update,
    Config {
        #[command(subcommand)]
        cmd: ConfigCommands
    },
    Reset
}
#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Name {
        #[arg(short, long)]
        name: String
    },
    Email {
        #[arg(short, long)] 
        email: String
    }
}
impl ConfigCommands {
    pub fn execute(&self) -> io::Result<()> {
        match self {
            ConfigCommands::Name { name } => Self::set_config_name(name),
            ConfigCommands::Email { email } => Self::set_config_email(email),
        }
    }
    pub fn set_config_name(name: &String) -> io::Result<()> {
        if !Path::new("./.udoc").exists() {
            panic!("This path is not a udoc repository"); 
        }
        
        let mut config_json: Config = config::read_config(
            "./.udoc/config.json".to_string()
        );
        config_json.user.username = name.to_string();
        std::fs::remove_file("./.udoc/config.json")?;
        config::create_config(
            &"./.udoc".to_string(),
            config_json
        ); 

        Ok(())
    }
    pub fn set_config_email(email: &String) -> io::Result<()> {
        if !Path::new("./.udoc").exists() {
            panic!("This path is not a udoc repository"); 
        }

        let mut config_json: Config = config::read_config(
            "./.udoc/config.json".to_string()
        );
        config_json.user.email = email.to_string();
        config::create_config(
            &"./.udoc".to_string(),
            config_json
        ); 

        Ok(())
    }

}
impl Commands {
     
    pub fn execute(&self) -> io::Result<()> {
        match self {
            Commands::New {
                name, 
                description, 
                image_dir_name, 
                video_dir_name, 
                error_number 
            } => Self::new(
                name, 
                description,
                image_dir_name,
                video_dir_name,
                error_number
            ),
            Commands::Update {} => Self::update(),
            Commands::Config { cmd } => cmd.execute(),
            Commands::Reset => Self::reset(),
        }
    }
    pub fn reset() -> io::Result<()> {
        if !Path::new("./.udoc").exists() {
            panic!("This is not a udoc repository ");
        }
        fs::remove_dir_all("./.udoc")
            .expect("TODO: panic message");
        config::create_config(
            &"./.udoc".to_string(),
            Config::new(
                1,
                "log.md".to_string(),
                "images".to_string(),
                "videos".to_string(),
                User {
                    username: String::new(),
                    email: String::new()
                }
            )
        ).expect("TODO: panic message");

        Ok(())
    }
    pub fn new(
        name: &String, 
        description: &String, 
        image_dir_name: &String, 
        video_dir_name: &String, 
        error_number: &u8) -> io::Result<()> {
        // Gets the current directory 
        let binding = std::env::current_dir()?;
        let root_path: Option<&str> = binding 
            .to_str();
        let date = Local::now()
            .date_naive(); 
        let error_name = format!("{name}-{date}");
        let root_path: &str = match root_path {
            Some(path) => path, 
            None => panic!("Path provided is None") 
        };

        let full_path: String = format!("{root_path}/{error_name}");
        let config_path: String = format!("{full_path}/.udoc");

        // Creates the directory
        fs::create_dir(&full_path);
        fs::create_dir(format!("{full_path}/.udoc"));
        config::create_config(
            &config_path,
            Config::new(
                1, 
                "log.md".to_string(), 
                format!("{image_dir_name}"),
                format!("{video_dir_name}"),
                User {
                    username: String::new(),
                    email: String::new()
                }
            )
        );
        let config_file = config::read_config(
            format!("{config_path}/config.json")
        );

        let images_dir = config_file.images_dir;
        let videos_dir = config_file.videos_dir;

        fs::create_dir(format!("{full_path}/{images_dir}"));
        fs::create_dir(format!("{full_path}/{videos_dir}"));
        
        // Creates the files
        log::create_log_file(&full_path, &error_name, description);

        Ok(())
    }

    pub fn update() -> io::Result<()> {
        let binding = std::env::current_dir()?;
        let path: Option<&str> = binding.to_str();
        let path: &str = match path {
            Some(path) => path, 
            None => panic!("Path provided is None")
        };
        let config_path = format!("{path}/.udoc");
        let config_json = config::read_config(
            format!("{config_path}/config.json"));
        if !Path::new(&config_path).exists() {
            panic!("This path is not a udoc repository"); 
        }
        else if config_json.user.email == String::new() || 
            config_json.user.username == String::new() {
            panic!("Please choose give your name and email");
        }
        
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{path}/log.md"))?;

        let root_path = path.to_string();
        log::update_images(&mut file, &root_path);

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::path::Path;

    fn setup() -> () {

    }
    fn destroy(name: String) -> () {
        std::fs::remove_dir_all(name);
        ()
    }

    #[test]
    fn init_test() -> io::Result<()> {

        let test_name: String = "test".to_string();
        let date = Local::now()
            .date_naive();       
        let full_path = format!("./{test_name}-{date}");

        let udoc_path = format!("{}/.udoc", &full_path);
        let images_path = format!("{}/images", &full_path);
        let videos_path = format!("{}/videos", &full_path);
        let log_file = format!("{}/log.md", &full_path);

        Commands::new(&test_name, &String::new());

        assert!(Path::new(&full_path).exists());
        assert!(Path::new(&udoc_path).exists());
        assert!(Path::new(&images_path).exists());
        assert!(Path::new(&videos_path).exists());
        assert!(Path::new(&log_file).exists());
        
        destroy(format!("{test_name}-{date}"));

        Ok(())
    }
}
