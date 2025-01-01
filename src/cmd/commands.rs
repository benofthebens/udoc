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

#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        #[arg(short, long)]
        name: String,

        #[arg(short, long, default_value_t = String::new())]
        description: String
    },
    Update {
        
    }
    
}

impl Commands {
     
    pub fn execute(&self) -> io::Result<()> {
        match self {
            Commands::New { name, description } => Self::new(name, description),
            Commands::Update {} => Self::update(),
        }
    }
    
    pub fn new(name: &String, description: &String) -> io::Result<()> {
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

        config::create_config(&config_path);
        let config_file = config::read_config(
            format!("{config_path}/config.json")
        );

        let images_dir = config_file.images_dir;
        let videos_dir = config_file.videos_dir;

        // Creates the directory
        fs::create_dir(&full_path);
        fs::create_dir(format!("{full_path}/.udoc"));
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

        if(!Path::new(&config_path).exists()) {
            panic!("This path is not a udoc repository"); 
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
