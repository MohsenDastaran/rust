use std::fs::File;
use std::process;

use std::io::{self, Read, stdin};
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

    let mut story = File::open(input.trim())?;

    let mut file_contents = String::new();

    story.read_to_string(&mut file_contents)?;

    return Ok(file_contents);
}
