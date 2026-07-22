use std::process;

fn main() {
    let a = 5;
    process::exit(0); // 0 means no error
    process::exit(1); // 01 or more means error: unreachable statement

    println!("sdv") // unreachable
}
