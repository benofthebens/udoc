pub mod exchange;
mod image;
mod section;
mod utils;

use std::cmp::PartialEq;
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
use crate::log::exchange::{Exchange, ExchangeItem};
pub use crate::log::image::{update_images, Image, Images};
use crate::log::section::Section;

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

    writeln!(file, "## {name}")?;
    writeln!(file, "---")?;
    writeln!(file, "### Description")?;
    writeln!(file, "{description}")?;
    writeln!(file, "---")?;
    writeln!(file, "### Examples")?;

    update_images(&mut file, root_path, config);

    Ok(())
}

pub fn read_log_file(log_path: &String) -> Result<Exchange, io::Error> {
    let log_file = File::open(log_path)?;
    let reader = BufReader::new(log_file);

    let mut exchange_file: Exchange = Default::default();
    let mut current_section: Section = Default::default();

    for line in reader.lines() {
        let line: String = line?;
        let pattern_vec: Vec<&str> = line.split(' ').collect();
        let mut pattern = match pattern_vec.get(0) {
            Some(pattern) => pattern,
            None => ""
        };
        if let Some(first_char) = line.chars().nth(0) {
            if first_char == '!' {
                pattern = "![";
            }
        }
        let exchange_type: ExchangeItem = ExchangeItem::map_markdown(pattern)
            .unwrap_or_else(|error| { panic!("") });

        if line.contains("### ") && !current_section.heading.is_empty() {
            exchange_file.section.push(current_section.clone());
            current_section = Default::default();
        }
        else if line.contains("---") {
            continue;
        }
        
        match exchange_type {
            ExchangeItem::Title => exchange_file.set_title(&line),
            ExchangeItem::Heading => current_section.handle_heading(&line),
            ExchangeItem::Text => current_section.set_text(&line),
            ExchangeItem::Image => current_section.set_images(&line),
        }

        println!("{:?}", current_section); 

    }
    exchange_file.section.push(current_section.clone());

    Ok(exchange_file)
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
