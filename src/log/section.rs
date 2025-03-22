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
    pub fn to_html(&self) -> Result<String, std::io::Error> {
        let mut html = String::new();

        html.push_str("<section>\n");
        html.push_str(format!("\t<h2>{}</h2>\n", self.heading).as_str());
        html.push_str(format!("\t<p>{}</p>\n", self.text).as_str());

        if let Some(images) = &self.images {
            html.push_str(images.to_html()?.as_str());
        }

        html.push_str("</section>\n");

        Ok(html)
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
