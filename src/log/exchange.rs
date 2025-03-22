mod html;

use std::{fs, io};
use quick_xml::de::from_str;
use quick_xml::se::Serializer;
use serde::{Deserialize, Serialize};
use crate::log::Section;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename = "error")]
pub struct Exchange {
    pub title: String,
    pub section: Vec<Section>,
}
impl Exchange {
    pub fn set_title(&mut self, title: &String) {
        let line = title
            .replace("## ", "")
            .to_lowercase();
        self.title = line;
    }
    pub fn to_html_file(&self) -> Result<(),std::io::Error>{
        let mut html = fs::read_to_string("./.udoc/exchange/template.html")?;
        println!("{}", html);
        let mut sections = String::new();

        &self.section.iter().for_each(|section| sections.push_str(
            section.to_html()
                .unwrap()
                .as_str()
        ));
        println!("{}", sections);

        html = html
            .replace("{{ title }}", &self.title)
            .replace("{{ sections }}", sections.as_str());

        fs::write("./exchange.html", html)?;
        Ok(())
    }
}
pub enum ExchangeItem {
    Title,
    Heading,
    Text,
    Image,
}
impl ExchangeItem {
    pub fn map_markdown<'a>(markdown_item: &str) -> Result<ExchangeItem, io::Error> {
           match markdown_item {
               "##" => Ok(ExchangeItem::Title),
               "###" => Ok(ExchangeItem::Heading),
               "![" => Ok(ExchangeItem::Image),
               _ => Ok(ExchangeItem::Text),
           }
    }
}
pub fn create_exchange_file(exchange_path: &String, contents: &Exchange) -> Result<String, io::Error> {
    let mut buffer = String::new(); // create a buffer to write to
    fs::create_dir_all(exchange_path)?;
    let mut ser = Serializer::with_root(&mut buffer, Some("error"))
        .unwrap();
    // Serialise the Exchange object to xml
    ser.indent(' ', 4);
    contents.serialize(ser).unwrap();
    fs::write(format!("{exchange_path}/exchange.xml"), &buffer)?;
    Ok(buffer)
}
pub fn read_exchange_file(exchange_path: &String) -> Result<Exchange, io::Error> {
    // Read the contents of the file into a string
    let file_path = format!("{}/exchange.xml", exchange_path);
    let file_content = fs::read_to_string(file_path)?;
    println!("{file_content}");

    // Deserialize the XML string into an Exchange struct
    let exchange: Exchange = from_str(&file_content).unwrap();

    Ok(exchange)
}
