use std::io; 
use std::fs;

pub struct Log {
    description: String,
}

impl Log {
    pub fn new(description: String) -> Self{
        Self {
            description
        }
    }

    pub fn get_images(&self, path: String) -> Vec<String> {
        let images = fs::read_dir(path)
            .expect("Unable to read directory");
        let mut image_list = vec![];
        
        for image in images {
            let entry: String = image
                .expect("Unable to get image")
                .path()
                .into_os_string()
                .into_string()
                .unwrap(); 
            image_list.push(entry);
        }
        image_list 
    }
}
pub fn create_log_file(path: &String) -> () {
        fs::write(
            format!("{path}/log.md"), 
            "".to_string()
        );
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_images_test() -> () {
        let log = Log::new(" ".to_string());
        dbg!("Hello"); 
        let images = log.get_images(".".to_string());
        for image in images {
            println!("{}", image);
        }
    }
}
