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
    pub fn create_log_file(&self, path: String) -> () {
        fs::write("./log.md", &self.description);
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
