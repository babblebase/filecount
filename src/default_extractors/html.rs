use std::error::Error;
use crate::{extract::Extract};
use std::str::from_utf8;

use html_parser::{Dom, Node};

pub struct Html;

fn extract_text_from_node(node: Node) -> Vec<String> {
    match node {
        Node::Text(t) => vec![t],
        Node::Element(e) => {
            let children_ref = &e.children;
            if children_ref.into_iter().any(|n| n.text() != None) {
                return vec![e.children.into_iter().map(|n| extract_text_from_node(n)).flatten().collect::<Vec<String>>().join(" ")];
            }
            e.children.into_iter().map(|n| extract_text_from_node(n)).flatten().collect()
        },
        Node::Comment(_) => Vec::new()
    }
}


impl Extract for Html {
    fn can_extract(&self, buf: &[u8], extension: Option<&str>) -> bool {
        extension == Some("html") || extension == Some("htmlx") || infer::text::is_html(buf)
    }

    fn extract(&self, buf: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
        let str = from_utf8(buf)?;        
        let dom = Dom::parse(str)?;
        let mut vec = Vec::new();
        
        for child in dom.children {
            vec.append(&mut extract_text_from_node(child))
        }

        Ok(vec)
    }
}