use std::fs::File;
use std::{fs, io};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use std::io::{Error, Write};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Images {
    #[serde(rename = "image")]
    pub image_list: Vec<Image>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Image {
    #[serde(rename = "@alt")]
    pub alt: String,
    #[serde(rename = "@src")]
    pub src: String,
}
impl Images {
    pub fn to_html(&self) -> Result<String, Error> {
        let mut html = String::new();
        for img in &self.image_list {
            html.push_str(&format!("\t<img src=\"{}\" alt=\"{}\"/>\n", img.src, img.alt));
        }
        Ok(html)
    }
}