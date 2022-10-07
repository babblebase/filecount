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

/// A translation memory optimized in a BinaryTree datastructure for performant analysis
/// Most functions defined in this struct have a segment and a hash variant. This is for convenience only,
/// the segment variants simply make use of the included hashing implementation.
#[derive(Serialize, Deserialize, Debug)]
pub struct HashedMemory (BTreeSet<u64>);

impl HashedMemory {

    /// Add a segment to the memory [O(log(n)]
    pub fn add(&mut self, segment: &str) {        
        self.add_hash(hash(segment));
    }

    /// Adds the hash of a segment to the memory [O(log(n)]
    pub fn add_hash(&mut self, hash: u64) {
        self.0.insert(hash);
    }

    /// Delete a segment from the memory [O(log(n)]
    pub fn delete(&mut self, segment: &str) -> bool {
        self.0.remove(&hash(segment))
    }

    /// Checks to see if a segment exists in the memory [O(log(n)]
    pub fn contains(&self, segment: &str) -> bool {
        self.contains_hash(&hash(segment))
    }

    /// Checks to see if the hash of a segment exists in the memory [O(log(n)]
    pub fn contains_hash(&self, hash: &u64) -> bool {
        self.0.contains(hash)
    }

    /// Instantiate a new empty HashedMemory
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    /// Create a HashedMemory from a .tmx file binary.
    /// # Examples
    /// ```
    /// let mut memfile = File::open("files/mem.tmx").unwrap();
    /// let mut memciphertext = Vec::new();
    /// memfile.read_to_end(&mut memciphertext).unwrap();  
    ///
    /// let memory = HashedMemory::from_tmx(&memciphertext).unwrap();
    /// ```
    /// # Errors
    /// MemoryParseError: Can be caused by invalid .tmx files. F.e. because the tmx lacks a header or srclang.
    /// Other errors: When parsing the .tmx file failed for other reasons.
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
