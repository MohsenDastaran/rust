fn main() {
    let cake: String = bake_cake();
    println!("I baked a {}!", cake);
}

fn bake_cake() -> String {
    String::from("Chocolate cake") //  last expression is returned
    // if we dont return the value, the value will be dropped
}
