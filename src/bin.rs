#[macro_use]
extern crate clap;
extern crate sassruist;

use clap::Arg;
use sassruist::convert_file;

fn main() {
    let app = app_from_crate!().arg(
        Arg::with_name("filepath")
            .help("target filepath")
            .required(true),
    );

    let matches = app.get_matches();
    if let Some(o) = matches.value_of("filepath") {
        println!("target filepath: {}", o);
        match convert_file(&o) {
            Ok(text) => {
                println!("{}", text);
            }
            Err(e) => println!("{}", e),
        }
    }
}
