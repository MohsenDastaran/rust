fn main() {
    let mut cofee = String::from("Mocha");
    let a = &mut cofee;

    let b = &a;

    println!("a: {}, b: {}", a, b);
}
