use std::error::Error;
use crate::{extract::Extract, xml_extensions::extract_text_from_node};
use std::str::from_utf8;

pub struct Xml;

impl Extract for Xml {
    fn can_extract(&self, buf: &[u8], _extension: Option<&str>) -> bool {
        infer::text::is_xml(buf)
    }

    fn extract(&self, buf: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
        let str = from_utf8(buf)?;        
        let doc = roxmltree::Document::parse(str)?;    
        Ok(extract_text_from_node(doc.root()))
    }
}