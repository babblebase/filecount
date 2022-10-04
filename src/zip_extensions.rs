use std::error::Error;
use std::io::{Cursor, Read};

pub fn read_file_from_zip(buf: &[u8], name: &str) -> Result<String, Box<dyn Error>> {
    let mut zip = zip::ZipArchive::new(Cursor::new(buf))?;
    let mut file = zip.by_name(name)?;
    let mut ciphertext = Vec::new();
    file.read_to_end(&mut ciphertext)?;
    let content = String::from_utf8(ciphertext)?;
    Ok(content)
}

pub fn read_files_from_zip(buf: &[u8], name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut vec = Vec::new();
    let mut zip = zip::ZipArchive::new(Cursor::new(buf))?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let file_name = file.name();            
        if !file_name.contains(name) {
            continue;
        }
        let mut ciphertext = Vec::new();
        file.read_to_end(&mut ciphertext)?;
        let file_contents = String::from_utf8(ciphertext)?;
        vec.push(file_contents);
    }
    Ok(vec)
}