pub mod converter;
pub mod parser;

pub fn convert_file(path: &str) -> Result<String, String> {
    match parser::parse_file(path) {
        Ok(content) => Ok(converter::perform(&content)),
        Err(e) => Err(format!("parse error: {}", e)),
    }
}
