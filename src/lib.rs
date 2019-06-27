pub mod converter;
pub mod parser;

pub fn convert_file(path: &str) -> Result<String, String> {
    match parser::parse_file(path) {
        Ok(content) => Ok(converter::perform(&content)),
        Err(e) => Err(format!("parse error: {}", e)),
    }
}

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn return_string(text: String) -> String {
    converter::perform(&text).into()
}
