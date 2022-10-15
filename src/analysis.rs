use crate::{memory::HashedMemory, segmentation::Hashment};
use core::convert::From;
use derive_more::{Add, AddAssign};
use std::fmt::Debug;
use serde::{Serialize, Deserialize};

/// Primitive struct to encapsulate the different analysis results.
#[derive(Default, Add, AddAssign, Debug, Serialize, Deserialize)]
pub struct Counts {
    /// The amount of analyzed segments
    pub segments: usize,

    /// The amount of analyzed words
    pub words: usize,

    /// The amount of analyzed characters (non-whitespace)
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

/// Wrapper around the different elements common in a translation analysis
#[derive(Serialize, Deserialize, Debug)]
pub struct Analysis {    
    /// The plain total counts. Not taking repetitions or matches into account.
    pub total: Counts,

    /// The counts for repeated segments.
    /// Given a hashment vector only containing 2 identical hashments, the total counts will be 2 and
    /// the repetitions will be 1 (as the second segment is a repeat of the first).
    pub repetitions: Counts,

    /// The counts for segments matched with the translation memory.
    pub matches: Counts
}

impl Analysis {

    /// Default constructor for analyses
    pub fn new() -> Analysis {
        Analysis { 
            total: Counts { segments: 0, words: 0, characters: 0 }, 
            repetitions: Counts { segments: 0, words: 0, characters: 0 },  
            matches: Counts { segments: 0, words: 0, characters: 0 },  
        }
    }
}

/// Given [hashments](Hashment) and an optional [hashed translation](HashedMemory) memory, this function will perform the final analysis.
/// # Example
/// ```
/// let mut memfile = File::open("files/mem.tmx").unwrap();
/// let mut memciphertext = Vec::new();
/// memfile.read_to_end(&mut memciphertext).unwrap();  
/// 
/// let mem = HashedMemory::from_tmx(&memciphertext).unwrap();
/// 
/// let mut file = File::open(&path).unwrap();
/// let mut ciphertext = Vec::new();
/// file.read_to_end(&mut ciphertext).unwrap();  
/// 
/// let texts = extract(ciphertext, &path, ExtractionRules::default()).unwrap();
/// let hashments = filecount::segmentation::hashment_many(texts, &UnicodeRules);
/// let analysis = analyze(hashments, &mem);
/// ```
pub fn analyze(hashments: &Vec<Hashment>, memory: &HashedMemory) -> Analysis {
    let mut repetition_memory = HashedMemory::new();
    let mut total = Counts::default();
    let mut repetitions = Counts::default();
    let mut matches = Counts::default();

    for hashment in hashments {
        total += Counts::from(hashment);

        if memory.contains_hash(&hashment.hash) {
            matches += Counts::from(hashment);
        }
        
        if repetition_memory.contains_hash(&hashment.hash) {
            repetitions += Counts::from(hashment);
        }

        repetition_memory.add_hash(hashment.hash);
    }

    Analysis {
        total,
        repetitions,
        matches
    }
}