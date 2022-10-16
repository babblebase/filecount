use std::error::Error;
use crate::{extract::Extract};
use std::str::from_utf8;

pub struct Xliff;

impl Extract for Xliff {
    fn can_extract(&self, _buf: &[u8], extension: Option<&str>) -> bool {
        extension == Some("xlf") || extension == Some("xliff")
    }

    fn extract(&self, buf: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
        let str = from_utf8(buf)?;
        let mut vec = Vec::new();          
        let doc = roxmltree::Document::parse(str)?;        
        for segment in doc.descendants().filter(|n| n.has_tag_name("segment") ) {
            if segment.descendants().any(|n| n.has_tag_name("target") && match n.text() {
                Some(x) => x.len() > 0,
                None => false,
            }) {
                continue;
            }
            for source in segment.descendants().filter(|n| n.has_tag_name("source")) {
                match source.text() {
                    Some(t) => vec.push(String::from(t)),
                    None => (),
                }
            }
        }
        Ok(vec)
    }
}