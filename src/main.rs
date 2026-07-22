use std::fs;
use std::process;

use std::io::{self, stdin};
fn main() {
    let content: Result<String, io::Error> = read_file();
    match content {
        Ok(i) => println!("{i}"),
        Err(error) => {
            eprintln!("Error Happened, File is missing: {error:?}");
            process::exit(1)
        }
    }
}

fn read_file() -> Result<String, io::Error> {
    println!("Please enter the file path");

    let mut input = String::new();

    stdin().read_line(&mut input)?;
    fs::read_to_string(input.trim())
}
