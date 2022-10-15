use crate::hash::hash;
use serde::{Serialize, Deserialize};

/// A hashment (hashed segment) represents the relevant information of an analyzed segment.
/// We turn segments into hashments in order to perform a time and memory optimized TM and repetition analysis.
#[derive(Serialize, Deserialize)]
pub struct Hashment {
    pub hash: u64,
    pub words: usize,
    pub characters: usize,
}

/// Trait to define custom segmentation rules.
pub trait SegmentationRules {
    /// Given a section of text, most likely containing multiple sentences, segment the text into translatable units.
    fn segment<'a>(&self, section: &'a str) -> Vec<&'a str>;

    /// The amount of words in a particular segment
    fn count_words(&self, segment: &str) -> usize;

    /// The amount of non-whitespace characters in a particular segment
    fn count_characters(&self, segment: &str) -> usize;
}

/// Hashmenting a section turns it into a vector of analyzable data.
/// Define and inject your own segmentation rules to modify the segmentation behaviour.
/// # Examples
/// ```
/// use filecount::unicode::UnicodeRules;
/// 
/// let hashments = filecount::segmentation::hashment("This is a sentence. This is another sentence.", &UnicodeRules);
/// assert_eq!(2, hashments.len());
/// ```
pub fn hashment<T: SegmentationRules>(section: &str, rules: &T) -> Vec<Hashment>  {
    let mut hashments = Vec::new();    

    for segment in rules.segment(section) {
        let word_count = rules.count_words(segment);

        if word_count <= 0 {
            continue;
        }

        let charachter_count = rules.count_characters(segment);

        hashments.push(Hashment {
            hash: hash(segment),
            words: word_count,
            characters: charachter_count,
        })
    }
    
    hashments
}

/// Utility function that allows you to hashment a vector of strings at once.
/// Often used in conjunction with extract().
/// Define and inject your own segmentation rules to modify the segmentation behaviour.
pub fn hashment_many<T: SegmentationRules>(sections: Vec<String>, rules: &T) -> Vec<Hashment> {
    sections.iter().flat_map(|s| hashment(s, rules)).collect()
}