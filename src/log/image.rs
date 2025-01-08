use std::fs::File;
use std::{fs, io};
use serde::Serialize;
use crate::config::Config;
use std::io::Write;

#[derive(Debug, Default, Serialize, Clone)]
pub struct Images {
    #[serde(rename = "image")]
    pub image_list: Vec<Image>,
}
#[derive(Debug, Default, Serialize, Clone)]
pub struct Image {
    #[serde(rename = "@alt")]
    pub alt: String,
    #[serde(rename = "@src")]
    pub src: String,
}
pub fn update_images(
    file: &mut File,
    root_path: &String,
    config: Config
) -> Result<(), io::Error> {
    let image_dir = config.images_dir;
    let images: Vec<String> = get_images(format!("{root_path}/{image_dir}"));

    for image in images {
        let image_file_path = image.replace("\\", "/");
        writeln!(file, "![Sample Image][{image_file_path}]")?;
    }

    Ok(())
}
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
