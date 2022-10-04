use std::error::Error;
use crate::extract::Extract;
use serde_json::{Value};

pub struct Json;

fn extract_text_from_json_value(v: &Value) -> Vec<String> {
    if v.is_string() {
        match v.as_str() {
            Some(w) => return vec![String::from(w)],
            None => return Vec::new(),
        }
    }

    if v.is_array() {
        match v.as_array() {
           Some(w) => return w.iter().flat_map(|x|extract_text_from_json_value(x)).collect(),
           None => return Vec::new(),
        }
    }

    if v.is_object() {
        match v.as_object() {
            Some(w) => {
                let mut vec = Vec::new();
                for (_, value) in w {
                    vec.append(&mut extract_text_from_json_value(value))
                }
                return vec;
            },
            None => return Vec::new()
        }
    }

    return Vec::new()
}

impl Extract for Json {
    fn can_extract(&self, _buf: &[u8], extension: Option<&str>) -> bool {
        match extension {
            Some(x) => x == "json",
            None => false,
        }
    }

    fn extract(&self, buf: &[u8]) -> Result<Vec<String>, Box<dyn Error>> {       
        let v: Value = serde_json::from_slice(buf)?;  
        Ok(extract_text_from_json_value(&v))
    }
}