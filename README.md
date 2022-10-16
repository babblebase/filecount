![crates.io](https://img.shields.io/crates/v/filecount.svg)

# 🧛 Filecount
Filecount is a modern high-performance open source file analysis library for automating localization tasks. It enables you to add file analysis functionality to your projects while maintaining a lot of customizability and extensibility. The hashment algorithm will always ensure optimal analysis performance.

Counting words is [a notoriously difficult problem](https://thehappybeavers.com/blog/why-word-count-differ-programs/) as it is really hard to define rules that give an "accurate" word count for every language. This means that many different text editing programs and CAT tools give different word counts for the same text! Filecount's philosophy is to be **fast and accurate enough**. Because for the purpose of having a fast file analysis it is often fine to be close enough to an accurate count.

If you want to see Filecount in action then visit the website: [Filecount.io](https://filecount.io/)

## Example
```rust
use std::env;
use std::io::Read;
use std::process;
use filecount::analysis::analyze;
use filecount::extract::{extract, ExtractionRules};
use filecount::segmentation::{hashment_many};
use filecount::memory::HashedMemory;
use filecount::unicode::UnicodeRules;
use std::fs::{File};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Not enough arguments passed. Please provide a path to a file or folder");
        process::exit(1);
    }

    let path = args[1].clone();

    let mut memfile = File::open("memory.tmx").unwrap();
    let mut memfile_buffer = Vec::new();
    memfile.read_to_end(&mut memfile_buffer).unwrap();  

    let memory = HashedMemory::from_tmx(&memfile_buffer).unwrap();

    let mut file = File::open(&path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();  

    let texts = extract(buffer, &path, ExtractionRules::default()).unwrap();
    let hashments = hashment_many(texts, &UnicodeRules);
    let analysis = analyze(hashments, &memory);
    println!("{:?}", analysis);       
}
```

## Usage
Filecount uses 3 basic principles, each represented by their respective function:
- extract
- hashment
- analyze

The extract function extracts textual elements from files supported by injected extraction rules. A set of default extraction rules for common file types is included.

The hashment function converts these extracted sections into hashed segments (hence hashment) with word and character counts given injected segmentation rules ([Unicode Standard Annex #29](http://www.unicode.org/reports/tr29/) supported by default).

The analyze function analyzes these hashments given an (optional) translation memory in order to get the total word and character counts, repetitions and TM matches.

Filecount deliberatly splits this functionality for optimal user control over the usage of these functions.

## Theoretical specifications
By storing segments in hashed format (see hashment in the documentation) in a binary tree, lookups will have a complexity of O(log N) where N is the size of the memory. This way a full file analysis can be performed in O(N log N) with N being the amount of segments in the file. Filecount deliberatly doesn't calculate fuzzy matches (50% TM match, 80% TM match, etc.) as these matches usually have less value to the file processor and this will ensure a high-performance operation. 

## Installation
Use this package in your project by adding the following
to your `Cargo.toml`:

```toml
[dependencies]
filecount = "0.1.0"
```

## Supported file formats
- docx
- pptx
- xlsx
- json
- xml
- txt
- xliff
- md
- html(x)

## Planned features
- Supporting many more default filetypes (including srt, doc, pdf, po, etc.) (all pull requests are welcome)
- In context matches (although different CAT tools use different definitions of 'in context')
- Adding seconds and minutes to analysis outputs for audiovisual files (relevant for subtitling related tasks)
- .srx based default segmentation support
- .xliff based .tmx and hashed memory management (using .xliff files to populate .tmx)
- Any file to .xliff conversion based on segmentation rules
- Reconverting translated .xliff files to their original filetypes