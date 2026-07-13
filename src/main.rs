fn main() {
    let number = -5;

    match number {
        2 | 4 | 6 | 8 => println!("Even"),
        1 | 3 | 5 | 7 => println!("Odd"),
        _ => println!("Unknown"),
    }

    match number {
        value if value % 2 == 0 => println!("Even"),
        _ => println!("Odd"),
    }
}
