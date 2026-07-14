fn main() {
    let person: String = String::from("Mohsen");
    let mo: String = person;

    // println!("{}", person);
    println!("{}", mo);

    // 1 memory but 2 refrences(pointers), who's responsible for cleaning up?
    // rust will move the ownership to second variable
}
