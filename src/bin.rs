#[macro_use]
extern crate clap;
extern crate sassruist;
use clap::Arg;
use std::fs::File;
use std::io::Write;

pub mod converter;
pub mod parser;

fn main() {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("filepath")
                .help("target filepath")
                .required(true),
        )
        .arg(
            Arg::with_name("overwrite")
                .help("overwrite original files")
                .short("o")
                .long("overwrite"),
        );

    let matches = app.get_matches();
    if let Some(o) = matches.value_of("filepath") {
        if matches.is_present("overwrite") {
            if let Err(e) = convert_and_write_file(o) {
                println!("{}", e);
            }
        } else {
            if let Err(e) = convert_file(o) {
                println!("{}", e);
            }
        }
    }
}

pub fn convert_file(path: &str) -> Result<(), String> {
    match parser::parse_file(path) {
        Ok(content) => {
            let text = converter::perform(&content);
            println!("{}", text);
            Ok(())
        }
        Err(e) => Err(format!("failed to parse: {}", e)),
    }
}

pub fn convert_and_write_file(path: &str) -> Result<(), String> {
    match parser::parse_file(path) {
        Ok(content) => {
            let result = converter::perform(&content);
            if content != result {
                match write_file(&result, path) {
                    Ok(_) => {
                        println!("update file: {}", path);
                        Ok(())
                    }
                    Err(e) => Err(format!("failed to write: {}", e)),
                }
            } else {
                // needless to update
                Ok(())
            }
        }
        Err(e) => Err(format!("failed to parse: {}", e)),
    }
}

fn write_file(text: &str, filepath: &str) -> Result<(), Box<std::error::Error>> {
    let mut file = File::create(filepath)?;
    write!(file, "{}", text)?;
    file.flush()?;
    Ok(())
}
