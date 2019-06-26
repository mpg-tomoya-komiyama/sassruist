use std::fs;

pub fn parse_file(filepath: &str) -> Result<String, Box<std::error::Error>> {
    let content = fs::read_to_string(filepath)?;
    Ok(content)
}
