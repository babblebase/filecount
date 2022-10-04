use std::collections::BTreeSet;
use serde::{Serialize, Deserialize};
use crate::hash::hash;
use std::error::Error;
use std::str::from_utf8;
use std::fmt;

#[derive(Debug, Clone)]
struct MemoryParseError(String);

impl Error for MemoryParseError {}

impl fmt::Display for MemoryParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct HashedMemory (BTreeSet<u64>);

impl HashedMemory {
    pub fn add(&mut self, segment: &str) {        
        self.add_hash(hash(segment));
    }

    pub fn add_hash(&mut self, hash: u64) {
        self.0.insert(hash);
    }

    pub fn delete(&mut self, segment: &str) -> bool {
        self.0.remove(&hash(segment))
    }

    pub fn contains(&self, segment: &str) -> bool {
        self.contains_hash(&hash(segment))
    }

    pub fn contains_hash(&self, hash: &u64) -> bool {
        self.0.contains(hash)
    }

    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn from_tmx(buf: &[u8]) -> Result<Self, Box<dyn Error>> {
        let mut mem = Self::new();
        let str = from_utf8(buf)?;
        let doc = roxmltree::Document::parse(str)?;   
        let header = doc.descendants().find(|n| n.has_tag_name("header")).ok_or(MemoryParseError(String::from("Missing header in tmx")))?;
        let srclang = header.attribute("srclang").ok_or(MemoryParseError(String::from("Missing srclang in tmx header")))?;

        for node in doc.descendants().filter(|n| n.has_tag_name("tuv") && n.attributes().iter().any(|a| a.name() == "lang" && a.value() == srclang)) {
            for seg in node.descendants().filter(|n| n.is_text()) {
                match seg.text() {
                    Some(t) => mem.add(t),
                    None => (),
                }
            }
        }
        Ok(mem)
    }
}
