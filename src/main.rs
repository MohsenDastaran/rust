fn main() {
    let first_name = {
        let actor: &str = "Keanu Reeves";
        &actor[0..8]
    };
    println!("Last Name: {}", first_name);
}
