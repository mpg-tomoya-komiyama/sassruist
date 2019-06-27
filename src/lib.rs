pub mod converter;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn return_string(text: String) -> String {
    converter::perform(&text).into()
}
