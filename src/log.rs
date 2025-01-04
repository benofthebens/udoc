use crate::config;
use config::Config;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn get_images(image_path: String) -> Vec<String> {
    let images = fs::read_dir(image_path).expect("Unable to read directory");
    let mut image_list = vec![];

    for image in images {
        let entry = image.expect("Unable to get image").path();
        if entry.extension().map(|s| s != "png").unwrap() {
            panic!("Incorrect file type for Image");
        }
        image_list.push(entry.into_os_string().into_string().unwrap());
    }
    image_list
}

pub fn create_log_file(
    path: &String,
    name: &String,
    description: &String,
) -> Result<(), io::Error> {

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
pub fn update_images(file: &mut File, root_path: &String) -> Result<(), io::Error> {

    let config_path: String = format!("{root_path}/.udoc/config.json");
    let config: Config = config::read_config(&config_path);
    let image_dir = config.images_dir;

    let images: Vec<String> = get_images(format!("{root_path}/{image_dir}"));

    for image in images {
        let image_file_path = image.replace("\\", "/");
        writeln!(file, "![Sample Image][{image_file_path}]")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() -> () {
        std::fs::create_dir("test");
        std::fs::create_dir("test/images");
        std::fs::create_dir(".udoc");
    }
    fn destroy() -> () {
        std::fs::remove_dir_all("test");
    }
    #[test]
    fn create_log_file_test() -> () {
        setup();

        let path: String = "C:/Users/benja/Documents/udoc/test".to_string();
        create_log_file(&path, &"err".to_string(), &String::new());

        assert!(Path::new(&format!("{path}/log.md")).exists());

        destroy();
    }
}
