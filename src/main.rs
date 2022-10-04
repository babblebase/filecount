use std::env;
use std::io::Read;
use std::process;
use file_count::analysis::analyze;
use file_count::extract::{extract, ExtractionRules};
use file_count::memory::HashedMemory;
use file_count::unicode::UnicodeRules;
use std::fs::{File};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments passed. Please provide a path to a file or folder");
        process::exit(1);
    }

    let path = args[1].clone();

    let mut memfile = File::open("files/mem.tmx").unwrap();
    let mut memciphertext = Vec::new();
    memfile.read_to_end(&mut memciphertext).unwrap();  

    let mem = HashedMemory::from_tmx(&memciphertext).unwrap();
    println!("{}", mem.contains("Hello world!"));

    let mut file = File::open(&path).unwrap();
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext).unwrap();  

    let texts = extract(ciphertext, Some(&path), ExtractionRules::default()).unwrap();
    let hashments = file_count::segmentation::hashment_many(texts, &UnicodeRules);
    let analysis = analyze(hashments, Some(&mem));
    println!("{:?}", analysis);

    // {
    //     let mut file = File::create("files/mem.bin").unwrap();
    //     bincode::serialize_into(&mut file, &mem).unwrap();
    // }

    // let mut file = File::open("files/mem.bin").unwrap();
    // let decoded: HashedMemory = bincode::deserialize_from(&mut file).unwrap();

    // println!("{}", decoded.contains("Hello world"));
    // println!("{}", decoded.contains("Hello world!"));
    // println!("{}", decoded.contains("Hello file count"));    
    
}


