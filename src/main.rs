fn countdown(n: i32) {
    if n > 0 {
        println!("{}!", n);
        countdown(n - 1);
    } else {
        println!("Liftoff!");
    }
}

fn main() {
    countdown(5);
}
