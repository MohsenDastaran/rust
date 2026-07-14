fn main() {
    let person: String = String::from("Mohsen");

    // this will not move ownership, it clones the value, different heap memory
    let mo: String = person.clone();

    println!("{}", person);
    println!("{}", mo);
}
