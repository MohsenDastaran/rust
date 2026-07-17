fn main() {
    let registrations: [bool; 3] = [true, false, true];
    let first: bool = registrations[0];

    println!(
        "First registration status: {} & {:#?}",
        first, registrations
    );

    // example for Array of heap allocated strings
    let langs: [String; 3] = [
        String::from("Rust"),
        String::from("Python"),
        String::from("JavaScript"),
    ];
    // error: String does not implement the `Copy` trait, so we cannot directly assign it to a variable like this. We need to clone it or use a reference.
    // let first_lang: String = langs[0];

    // Correct way to get the first language because we are borrowing it, not taking ownership
    let first_lang: &String = &langs[0];

    // or clone it if you need ownership
    // let first_lang: String = langs[0].clone();

    println!("First Language status: {} & {:#?}", first_lang, langs);
}
