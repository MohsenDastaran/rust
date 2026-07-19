fn main() {
    let instruments = [
        String::from("Guitar"),
        String::from("Piano"),
        String::from("Tar"),
    ];

    let base: Option<&String> = instruments.get(10);
    println!("{base:?}")
}
