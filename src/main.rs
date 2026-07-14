fn main() {
    let person: String = String::from("Mohsen");
    let mo: String = person;

    // no drop:
    println!("{}", mo);

    // drop manually
    drop(mo);

    // error:
    // println!("{}", mo);
} // auto drop on closing curly bracket
