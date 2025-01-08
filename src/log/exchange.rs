use std::{fs, io};
use quick_xml::se::Serializer;
use serde::Serialize;
use crate::log::Section;

#[derive(Debug, Default, Serialize)]
pub struct Exchange {
    pub title: String,
    pub sections: Vec<Section>,
}
impl Exchange {
    pub fn set_title(&mut self, title: &String) {
        let line = title
            .replace("## ", "")
            .to_lowercase();
        self.title = line;
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