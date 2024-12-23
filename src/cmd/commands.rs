use std::fs::File;
use std::fs;
use std::io;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {
    
    New {
        #[arg(short, long)]
        path: String,
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

        let test_path: String = "C:/Users/benja/Documents/udoc".to_string();
        let test_name: String = "test".to_string();
        let full_path = format!("{}/{}", test_path, test_name);
        
        let udoc_path = format!("{}/.udoc", &full_path);
        let images_path = format!("{}/images", &full_path);
        let videos_path = format!("{}/videos", &full_path);

        Commands::new(&test_path, &test_name);
        
        assert!(Path::new(&full_path).exists());
        assert!(Path::new(&udoc_path).exists());
        assert!(Path::new(&images_path).exists());
        assert!(Path::new(&videos_path).exists());

        fs::remove_dir_all(&full_path)
            .expect("Unable to delete directory");

        Ok(())
    }
}
impl Commands {
    pub fn execute(&self) -> io::Result<()>{
        match self {
            Commands::New { path, name } => Self::new(path, name)
        }
    }
    pub fn new(path: &String, name: &String) -> io::Result<()> {

        let full_path: String = format!("{path}/{name}");

        fs::create_dir(&full_path);
        fs::create_dir(format!("{}/.udoc", &full_path));
        fs::create_dir(format!("{}/images", &full_path));
        fs::create_dir(format!("{}/videos", &full_path));
        
        Ok(())
    }
}

