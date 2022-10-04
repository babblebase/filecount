use std::error::Error;
use crate::{extract::Extract, zip_extensions::read_file_from_zip, xml_extensions::extract_text_from_nodes};

pub struct Xlsx;

impl Extract for Xlsx {
    fn can_extract(&self, buf: &[u8], _extension: Option<&str>) -> bool {
        infer::doc::is_xlsx(buf)
    }

    fn extract(&self, buf: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
        let shared_strings = read_file_from_zip(buf, "xl/sharedStrings.xml")?;
        let doc = roxmltree::Document::parse(&shared_strings)?;
        let texts = doc.root().descendants().filter(|n| n.has_tag_name("t")).collect();
        Ok(extract_text_from_nodes(texts))
    }
}