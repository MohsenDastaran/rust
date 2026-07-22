use std::fs::File;
use std::process;

use std::io::stdin;
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
        println!("Something Went Wrong!");
        process::exit(2)
    }

    let story: File = match File::open(input.trim()) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error Happened, File is missing: {error:?}. user output: {input}");
            process::exit(1)
        }
    };
    println!("{:?}", story);
}
