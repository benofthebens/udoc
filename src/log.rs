mod exchange;

use crate::config;
use config::Config;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use quick_xml::se::Serializer;
use serde::Serialize;

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
    root_path: &String,
    name: &String,
    description: &String,
    config: Config
) -> Result<(), io::Error> {

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{root_path}/log.md"))?;

    writeln!(file, "## {name}");
    writeln!(file, "---");
    writeln!(file, "### Description");
    writeln!(file, "{description}");
    writeln!(file, "---");
    writeln!(file, "### Examples");

    update_images(&mut file, root_path, config);

    Ok(())
}

#[derive(Debug, Default, Serialize)]
pub struct Exchange {
   title: String,
   sections: Vec<Section>,
}
#[derive(Debug, Default, Serialize, Clone)]
pub struct Section {
    heading: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    images: Option<Images>,
}
#[derive(Debug, Default, Serialize, Clone)]
pub struct Images {
    #[serde(rename = "image")]
    image_list: Vec<Image>,
}
#[derive(Debug, Default, Serialize, Clone)]
pub struct Image {
    #[serde(rename = "@alt")]
    alt: String,
    #[serde(rename = "@src")]
    src: String,
}
pub fn read_log_file(log_path: &String) -> Result<Exchange, io::Error> {
    let log_file = File::open(log_path)?;
    let reader = BufReader::new(log_file);

    let mut exchange_file: Exchange = Default::default();
    let mut current_section: Section = Default::default();
    let mut image_list: Images = Default::default();

    for line in reader.lines() {
        let line: String  = line?;
        if line.contains("### ") && !current_section.text.is_empty() {
            exchange_file.sections.push(current_section.clone());
            current_section = Default::default();
            image_list = Default::default(); // Reset image list when a new section starts
        }

        if line.contains("### ") {
            let line = line
                .replace("### ", "")
                .to_lowercase();

            current_section.heading = line;
        }
        else if line.is_empty() || line.contains("---") {
            continue;
        }
        else if line.contains("## ") {
            let line = line.replace("## ", "");
            exchange_file.title = line;
        }
        else if line.contains("![") {
            let image = line.replace("!", "");
            let image: Vec<&str> = image
                .split(&['[',']'][..])
                .filter(|s| !s.is_empty())
                .collect();

            match current_section.images {
                Some(ref mut images) => {
                    images.image_list.push(
                        Image {
                            alt: image[0].to_string(),
                            src: image[1].to_string(),
                        }
                    );
                }
                None => {
                    current_section.images = Some(Images {
                        image_list: vec![Image {
                            alt: image[0].to_string(),
                            src: image[1].to_string(),
                        }],
                    });
                }
            }
        }
        else {
            current_section.text += &line;
        }
    }
    // Don't forget to add the last section
    exchange_file.sections.push(current_section.clone());

    Ok(exchange_file)
}

pub fn create_exchange_file(contents: Exchange) -> Result<String, io::Error> {
    let mut buffer = String::new();
    fs::create_dir_all("./.udoc/exchange")?;
    let mut ser = Serializer::with_root(&mut buffer, Some("error"))
        .unwrap();
    ser.indent(' ', 4);
    contents.serialize(ser).unwrap();
    fs::write("./.udoc/exchange/exchange.xml", &buffer)?;

    Ok(buffer)
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

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() {}
    fn teardown() {}
    #[test]
    fn update_images_test() {}
    #[test]
    fn create_log_file_test() {}
    #[test]
    fn get_images_test() {}
}
