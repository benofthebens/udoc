use std::io::BufRead;
use serde::{Deserialize, Serialize};
use crate::log::image::{Image, Images};

#[derive(Debug, Default, Serialize, Clone, Deserialize)]
pub struct Section {
    pub heading: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Images>,
}
impl Section {
    pub fn handle_heading(&mut self, line: &String) {
        let line = line
            .replace("### ", "")
            .to_lowercase();

        self.heading = line;
    }
    pub fn set_text(&mut self, text: &String) {
        self.text += text;
    }
    pub fn set_images(&mut self, line: &String) {
        let image = line.replace("!", "");
        let image: Vec<&str> = image
            .split(&['[',']','(', ')'][..])
            .filter(|s| !s.is_empty())
            .collect();

        match self.images {
            Some(ref mut images) => {
                images.image_list.push(
                    Image {
                        alt: image[0].to_string(),
                        src: image[1]
                            .to_string()
                    }
                );
            }
            None => {
                self.images = Some(Images {
                    image_list: vec![Image {
                        alt: image[0].to_string(),
                        src: image[1].to_string(),
                    }],
                });
            }
        }
    }
}
