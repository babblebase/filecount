use unicode_segmentation::UnicodeSegmentation;
use crate::segmentation::SegmentationRules;

pub struct UnicodeRules;

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

pub fn segment(section: &str) -> Vec<&str> {
    section.split_sentence_bounds().collect()
}

pub fn count_words(segment: &str) -> usize {
    segment.unicode_words().count()
}

pub fn count_characters(segment: &str) -> usize {
    segment.chars().filter(|c| !c.is_whitespace()).count()
}
