use crate::{memory::HashedMemory, segmentation::Hashment};
use core::convert::From;
use derive_more::{Add, AddAssign};
use std::fmt::Debug;

#[derive(Default, Add, AddAssign, Debug)]
pub struct Counts {
    pub segments: usize,
    pub words: usize,
    pub characters: usize,
} 

impl From<&Hashment> for Counts {
    fn from(hashment: &Hashment) -> Self {
        Counts {
            segments: 1,
            words: hashment.words,
            characters: hashment.characters,
        }
    }
}

#[derive(Debug)]
pub struct Analysis {    
    pub total: Counts,
    pub repetitions: Counts,
    pub matches: Counts
}


pub fn analyze(hashments: Vec<Hashment>, memory: Option<&HashedMemory>) -> Analysis {
    let mut repetition_memory = HashedMemory::new();
    let mut total = Counts::default();
    let mut repetitions = Counts::default();
    let mut matches = Counts::default();

    for hashment in hashments {
        total += Counts::from(&hashment);

        match memory {
            Some(m) => {
                if m.contains_hash(&hashment.hash) {
                    matches += Counts::from(&hashment);
                }
            }
            None => ()
        }
        
        if repetition_memory.contains_hash(&hashment.hash) {
            repetitions += Counts::from(&hashment);
        }

        repetition_memory.add_hash(hashment.hash);
    }

    Analysis {
        total,
        repetitions,
        matches
    }
}