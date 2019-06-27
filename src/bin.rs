#[macro_use]
extern crate clap;
extern crate regex;
extern crate sassruist;
extern crate walkdir;
use clap::Arg;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;

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
                .help("overwrite original file(s)")
                .short("o")
                .long("overwrite"),
        )
        .arg(
            Arg::with_name("directory")
                .help("batch execute to the directory")
                .short("d")
                .long("directory"),
        );

    let matches = app.get_matches();

    if matches.is_present("directory") {
        if let Some(o) = matches.value_of("filepath") {
            let re = Regex::new(r"\.(?i)(sass|scss)$").unwrap();
            for entry in WalkDir::new(o).into_iter().filter_map(|e| e.ok()) {
                let path = format!("{}", entry.path().display());
                if re.captures(&path).is_some() {
                    if matches.is_present("overwrite") {
                        if let Err(e) = convert_and_write_file(&path) {
                            println!("{}", e);
                            return;
                        }
                    } else {
                        if let Err(e) = convert_file(&path) {
                            println!("{}", e);
                            return;
                        }
                    }
                }
            }
        }
    } else {
        if let Some(path) = matches.value_of("filepath") {
            if matches.is_present("overwrite") {
                if let Err(e) = convert_and_write_file(path) {
                    println!("{}", e);
                    return;
                }
            } else {
                if let Err(e) = convert_file(path) {
                    println!("{}", e);
                    return;
                }
            }
        }
    }
}

pub fn convert_file(path: &str) -> Result<(), String> {
    match parser::parse_file(path) {
        Ok(content) => {
            let text = converter::perform(&content);
            println!("==========\n{}\n==========\n{}", path, text);
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
