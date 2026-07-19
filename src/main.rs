fn main() {
    let instruments: [String; 3] = [
        String::from("Guitar"),
        String::from("Piano"),
        String::from("Tar"),
    ];

    let base: Option<&String> = instruments.get(20);
    println!("{base:?}");
    // println!("{}", base.unwrap()); // to get the raw value
    println!("{}", base.expect("Error Happened Mohsen")) // to get the raw value with Error handling
}
