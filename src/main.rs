use std::fs::File;
use std::process;
fn main() {
    // std::io::Error is for when the file doesnt exist
    let story: File = match File::open("./story.txt") {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error Happened, File is missing");
            process::exit(1)
        }
    };
    println!("{:?}", story);
}
