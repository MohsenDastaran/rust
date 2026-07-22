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

    // another better way of error handling:
    let user_req_file = stdin().read_line(&mut input);

    // if error hapened
    if let Err(error) = user_req_file {
        return Err(error);
    }

    let mut story: File = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    // println!("{:?}", story);

    let mut file_contents = String::new();
    let read_operation = story.read_to_string(&mut file_contents);
    if let Err(error) = read_operation {
        return Err(error);
    }
    // println!("{}", file_contents);
    return Ok(file_contents);
}
