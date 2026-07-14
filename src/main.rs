fn main() {
    // store in binary executable
    let mut food: &str = "pasta";
    println!("{food}");

    food = "xfgcxfg";
    println!("{food}");

    // store in heap, its gonna change
    let text: String = String::new();
    let candy = String::from("Kitkat");

    println!("{candy}");
}
