#[macro_use]
extern crate clap;
extern crate regex;
extern crate walkdir;
use clap::Arg;
use regex::Regex;
use std::fs::metadata;
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;

pub mod converter;
pub mod parser;

fn main() {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("path")
                .help("target file or directory path")
                .required(true),
        )
        .arg(
            Arg::with_name("fix")
                .help("fix original file(s)")
                .short("f")
                .long("fix"),
        );

    let matches = app.get_matches();
    let is_fix = matches.is_present("fix");

    if let Some(path) = matches.value_of("path") {
        if is_file(&path) {
            if let Err(e) = perform_file(path, is_fix) {
                println!("some error occurred in: {}\n{}", path, e);
                return;
            }
        } else {
            let re = Regex::new(r"\.(?i)(sass|scss)$").unwrap();
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                let target_path = format!("{}", entry.path().display());
                if re.captures(&target_path).is_some() {
                    if let Err(e) = perform_file(&target_path, is_fix) {
                        println!("some error occurred in: {}\n{}", path, e);
                        return;
                    }
                }
            }
            println!("\ndone.");
        }
    }
}

fn perform_file(file: &str, is_fix: bool) -> Result<(), String> {
    if is_fix {
        if let Err(e) = convert_and_write_file(file) {
            return Err(format!("{}", e));
        }
    } else {
        if let Err(e) = convert_file(file) {
            return Err(format!("{}", e));
        }
    }
    Ok(())
}

fn is_file(path: &str) -> bool {
    if let Ok(meta) = metadata(path) {
        meta.is_file()
    } else {
        false
    }
}

fn convert_file(path: &str) -> Result<(), String> {
    match parser::parse_file(path) {
        Ok(content) => {
            let text = converter::perform(&content);
            println!("==========\n{}\n==========\n{}", path, text);
            Ok(())
        }
        Err(e) => Err(format!("failed to parse: {}", e)),
    }
}

fn convert_and_write_file(path: &str) -> Result<(), String> {
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
