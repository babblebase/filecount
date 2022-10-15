use std::{default::Default};
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::ffi::OsStr;

use crate::default_extractors::{txt,xml,docx,json,pptx,xlsx,xliff};

/// Thrown when parsing a file fails
#[derive(Debug)]
pub struct ExtractionError(String);

impl fmt::Display for ExtractionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ExtractionError {}

/// This trait is used to define extraction rule structs
pub trait Extract {
    /// Defines whether this rule can actually extract a file given the content and extension
    fn can_extract(&self, buf: &[u8], extension: Option<&str>) -> bool;

    /// The extraction logic, parses the file and extracts sections of translatable text.
    fn extract(&self, buf: &[u8]) -> Result<Vec<String>, Box<dyn Error>>;
}

/// Wrapper around implementations of [Extract](Extract) trait. Add custom extraction rules or use the [default extraction rules](ExtractionRules).
pub struct ExtractionRules {
    rules: Vec<Box<dyn Extract>>,
}

impl ExtractionRules {

    /// Add an [Extract](Extract) implementation to the ruleset
    pub fn add(&mut self, rule: Box<dyn Extract>) {
        self.rules.push(rule);
    }

    /// Instantiate a new [ExtractionRules](ExtractionRules) set
    pub fn new() -> Self {
        Self {
            rules: Vec::new()
        }
    }
}

/// The default implementation of [ExtractionRules](ExtractionRules) uses all the rules in the default_extractors folder
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

/// Given a file, path and a set of [extraction rules](ExtractionRules), this method will extract all the
/// sections of translatable text from the file. E.g. paragraphs from word documents, values from json files, 
/// untranslated source segments from xliff, etc.
/// 
/// The default implementation of [extraction rules](ExtractionRules) can be used, but custom extraction rules can also be defined for
/// files types that are not supported. (See [extraction rules](ExtractionRules))
/// # Examples
/// ```
/// let mut file = File::open(&path).unwrap();
/// let mut ciphertext = Vec::new();
/// file.read_to_end(&mut ciphertext).unwrap();  
///
/// let texts = extract(ciphertext, &path, ExtractionRules::default()).unwrap();
/// ```
/// # Errors
/// [ExtractionError](ExtractionError): No rule matched the file and/or path
/// 
pub fn extract(buf: Vec<u8>, path: &str, rules: ExtractionRules) -> Result<Vec<String>, Box<dyn Error>> {
    let extension = Path::new(path).extension().and_then(OsStr::to_str);
    for rule in rules.rules {
        if rule.can_extract(&buf, extension) {
            return rule.extract(&buf)
        }
    }
    Err(Box::new(ExtractionError(String::from("No rule matched file type"))))
}