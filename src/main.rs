fn main() {
    let age: i8 = 29;

    {
        let is_handsome: bool = true;
        // is_handsome is in memory here
    }
    // println!("{is_handsome}")
    // is_handsome is not in memory because its out of scope
}
