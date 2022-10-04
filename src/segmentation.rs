use crate::hash::hash;

pub struct Hashment {
    pub hash: u64,
    pub words: usize,
    pub characters: usize,
}

pub trait SegmentationRules {
    fn segment<'a>(&self, section: &'a str) -> Vec<&'a str>;
    fn count_words(&self, segment: &str) -> usize;
    fn count_characters(&self, segment: &str) -> usize;
}

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

pub fn hashment_many<T: SegmentationRules>(sections: Vec<String>, rules: &T) -> Vec<Hashment> {
    sections.iter().flat_map(|s| hashment(s, rules)).collect()
}