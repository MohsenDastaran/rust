fn main() {
    let actor: String = String::from("Keanu Reeves");

    let string_ref: &str = &actor[0..8];

    println!("Actor: {}", string_ref);

    let last_name = &actor[6..];
    println!("Last Name: {}", last_name);
}
