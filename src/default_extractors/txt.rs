use std::error::Error;
use crate::extract::Extract;
use std::str::from_utf8;

pub struct Txt;

impl Extract for Txt {
    fn can_extract(&self, _buf: &[u8], extension: Option<&str>) -> bool {
        match extension {
            Some(x) => x == "txt",
            None => false,
        }
    }

    fn extract(&self, buf: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {
        let s = from_utf8(buf)?;
        Ok(vec![String::from(s)])
    }
}