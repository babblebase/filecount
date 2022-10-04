use std::error::Error;
use crate::{extract::Extract, zip_extensions::read_file_from_zip};

pub struct Docx;

impl Extract for Docx {
    fn can_extract(&self, buf: &[u8], _extension: Option<&str>) -> bool {
        infer::doc::is_docx(buf)
    }

    fn extract(&self, buf: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
        let file = read_file_from_zip(buf, "word/document.xml")?;
        let doc = roxmltree::Document::parse(&file)?;
        let paragraphs = doc.root().descendants().filter(|n| n.has_tag_name("p"));
        let mut vec = Vec::new();
        for paragraph in paragraphs {
            let mut s = String::new();
            for node in paragraph.descendants() {
                if node.is_text() {
                    match node.text() {
                        Some(t) => s += t,
                        None => (),
                    }
                } else if node.has_tag_name("tab") {
                    s += "  ";
                }
            }
            vec.push(s);
        }

        Ok(vec)
    }
}