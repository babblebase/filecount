use unicode_segmentation::UnicodeSegmentation;
use crate::segmentation::SegmentationRules;

/// As a standard default segmentation ruleset Filecount uses the rust unicode implementation of [Unicode Standard Annex #29](http://www.unicode.org/reports/tr29/).
/// For more information see [the unicode-segmentation crate](https://crates.io/crates/unicode-segmentation)
pub struct UnicodeRules;

/// The Segmentation rules implementation for UnicodeRules
impl SegmentationRules for UnicodeRules {
    fn segment<'a>(&self, section: &'a str) -> Vec<&'a str> {
        segment(section)
    }

    fn count_words(&self, segment: &str) -> usize {
        count_words(segment)
    }

    fn count_characters(&self, segment: &str) -> usize {
        count_characters(segment)
    }
}

/// Split a section into multiple string references by unicode segmentation rules
pub fn segment(section: &str) -> Vec<&str> {
    section.split_sentence_bounds().collect()
}

// Counts the words in a segment according to unicode rules
pub fn count_words(segment: &str) -> usize {
    segment.unicode_words().count()
}

// Counts the characters in a segment naïvely, only counting non-whitespace characters
pub fn count_characters(segment: &str) -> usize {
    segment.chars().filter(|c| !c.is_whitespace()).count()
}
