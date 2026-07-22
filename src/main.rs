use std::fs::File;
use std::process;

use std::io::{Read, stdin};
fn main() {
    println!("Please enter the file path");

    let mut input = String::new();

    // match stdin().read_line(&mut input) {
    //     Ok(size) => println!("The Size!!!"),
    //     Err(error) => {
    //         println!("Something Went Wrong!");
    //         process::exit(2)
    //     }
    // }

    // another better way of error handling:
    let user_req_file = stdin().read_line(&mut input);

    // if error hapened
    if let Err(_error) = user_req_file {
        eprintln!("Something Went Wrong!");
        process::exit(2)
    }

    let mut story: File = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error Happened, File is missing: {error:?}. user output: {input}");
            process::exit(1)
        }
    };
    println!("{:?}", story);

    let mut file_contents = String::new();
    let read_operation = story.read_to_string(&mut file_contents);
    if let Err(_error) = read_operation {
        eprintln!("Something Went Wrong!");
        process::exit(2)
    }
    println!("{}", file_contents);
}
