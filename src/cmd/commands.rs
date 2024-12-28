use std::fs::File;
use std::fs;
use std::io;

use clap::Subcommand;

use crate::config::Config;
use crate::config;
use crate::log; 

#[derive(Subcommand, Debug)]
pub enum Commands {
    New {
        #[arg(short, long)]
        name: String
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn init_test() -> io::Result<()> {

        let test_name: String = "test".to_string();
        let full_path = format!("{}/{}", test_path, test_name);
        
        let udoc_path = format!("{}/.udoc", &full_path);
        let images_path = format!("{}/images", &full_path);
        let videos_path = format!("{}/videos", &full_path);
        let log_file = format!("{}/log.md", &full_path);

        Commands::new(&test_name);
        
        assert!(Path::new(&full_path).exists());
        assert!(Path::new(&udoc_path).exists());
        assert!(Path::new(&images_path).exists());
        assert!(Path::new(&videos_path).exists());
        assert!(Path::new(&log_file).exists());

        Ok(())
    }
}
impl Commands {
     
    pub fn execute(&self) -> io::Result<()> {
        match self {
            Commands::New { name } => Self::new(name)
        }
    }
    
    pub fn new(name: &String) -> io::Result<()> {
        
        let binding = std::env::current_dir()?;
        let root_path: Option<&str> = binding 
            .to_str();

        let root_path: &str = match root_path {
            Some(path) => path, 
            None => panic!("Path provided is None") 
        };

        let full_path: String = format!("{root_path}/{name}");
        let config_path: String = format!("{full_path}/.udoc");

        fs::create_dir(&full_path);
        fs::create_dir(format!("{}/.udoc", &full_path));
        fs::create_dir(format!("{}/images", &full_path));
        fs::create_dir(format!("{}/videos", &full_path));
        
        config::create_config(config_path);
        log::create_log_file(&full_path);

        Ok(())
    }
}

