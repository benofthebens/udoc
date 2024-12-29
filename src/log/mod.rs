use std::io; 
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
use std::path::Path;
pub struct Log {
    description: String,
}

impl Log {
    pub fn new(description: String) -> Self {
        Self {
            description
        }
   }

    pub fn get_images(path: String) -> Vec<String> {
        let images = fs::read_dir(path)
            .expect("Unable to read directory");
        let mut image_list = vec![];
        
        for image in images {
            let entry = image
                .expect("Unable to get image")
                .path();
            if entry.extension().map(|s| s != "png").unwrap() {
                panic!("Incorrect file type for Image");
            }
            image_list.push(entry
                .into_os_string()
                .into_string()
                .unwrap());
        }
        image_list 
    }
}

pub fn create_log_file(
    path: &String, 
    name: &String,
    description: &String) -> Result<(), std::io::Error> {

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{path}/log.md"))?;

    writeln!(file, "## {name}"); 
    writeln!(file, "---");
    writeln!(file, "### Description");
    writeln!(file, "{description}");
    writeln!(file, "---");
    writeln!(file, "### Examples");
    update_images(&mut file, path);
    Ok(())
}
pub fn update_images (
    file: &mut File,
    path: &String
    ) -> Result<(), std::io::Error> {

    let images: Vec<String> = Log::get_images(format!("{path}/images"));
    for image in images {
        let image_upd = image.replace("\\", "/");

        writeln!(
            file,
            "![Sample Image][{image_upd}]"
        )?;
    }

    Ok(())
}

/*#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_log_file_test() -> Result<()> {
        let path: String = "C:/Users/benja/Documents/udoc/test"
            .to_string();
        create_log_file(&path);

        assert!(Path::new(format!("{path}/log.md")).exists());

        Ok(())
    }

    #[test]
    fn get_images_test() -> () {
        let log = Log::new(" ".to_string());
        let images = log.get_images(".".to_string());
        for image in images {
            println!("{}", image);
        }
    }
}*/
