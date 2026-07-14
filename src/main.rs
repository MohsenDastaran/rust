fn main() {
    // store in heap, its gonna change
    let mut candy: String = String::from("Kitkat");
    println!("{candy}");

    candy.push_str(" Candy");

    println!("{candy}");

    // if text is too long, it will move to the new heap location automatically
    candy.push_str(" Candy fgdfgdfgdfgdfgdg");
    println!("{candy}");
}
