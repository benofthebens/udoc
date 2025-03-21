use std::fs::File;
use std::{fs, io};
use serde::{Deserialize, Serialize};
use crate::config::Config;
use std::io::Write;

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
