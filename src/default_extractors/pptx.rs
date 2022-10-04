use std::error::Error;
use crate::{extract::Extract, zip_extensions::read_files_from_zip, xml_extensions::extract_text_from_nodes};

pub struct Pptx;

impl Extract for Pptx {
    fn can_extract(&self, buf: &[u8], _extension: Option<&str>) -> bool {
        infer::doc::is_pptx(buf)
    }

    fn extract(&self, buf: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
        let mut vec = Vec::new();
        let slides = read_files_from_zip(buf, "ppt/slides/slide")?;
        for slide in slides {
            let doc = roxmltree::Document::parse(&slide)?;
            let mut texts = extract_text_from_nodes(doc.root().descendants().filter(|n| n.has_tag_name("t")).collect());
            vec.append(&mut texts)
        }
        Ok(vec)
    }
}