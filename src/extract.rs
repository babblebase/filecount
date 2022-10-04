use std::{default::Default};
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::ffi::OsStr;

use crate::default_extractors::{txt,xml,docx,json,pptx,xlsx,xliff};

#[derive(Debug)]
struct ExtractionError(String);

impl fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ExtractionError {}

pub trait Extract {
    fn can_extract(&self, buf: &[u8], extension: Option<&str>) -> bool;
    fn extract(&self, buf: &[u8]) -> Result<Vec<String>, Box<dyn Error>>;
}

pub struct ExtractionRules {
    rules: Vec<Box<dyn Extract>>,
}

impl ExtractionRules {
    pub fn add(&mut self, rule: Box<dyn Extract>) {
        self.rules.push(rule);
    }

    pub fn new() -> Self {
        Self {
            rules: Vec::new()
        }
    }
}

impl Default for ExtractionRules {
    fn default() -> Self {
        let mut new = ExtractionRules::new();
        new.add(Box::new(xliff::Xliff));
        new.add(Box::new(txt::Txt));
        new.add(Box::new(xml::Xml));
        new.add(Box::new(docx::Docx));
        new.add(Box::new(json::Json));
        new.add(Box::new(pptx::Pptx));
        new.add(Box::new(xlsx::Xlsx));
        new
    }
}

pub fn extract(buf: Vec<u8>, path: Option<&str>, rules: ExtractionRules) -> Result<Vec<String>, Box<dyn Error>> {
    let extension = path.and_then(|path| Path::new(path).extension().and_then(OsStr::to_str));
    for rule in rules.rules {
        if rule.can_extract(&buf, extension) {
            return rule.extract(&buf)
        }
    }
    Err(Box::new(ExtractionError(String::from("No rule matched file type"))))
}