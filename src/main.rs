// with error[E0382]: borrow of moved value: `apples`

// fn main() {
//     let apples: String = String::from("apples");

//     print_hello(apples);
//     println!("Hello 2, {}!", apples);
// }
// fn print_hello(value: String) {
//     println!("Hello, {}!", value);
// }

// without error:
fn main() {
    let apples: String = String::from("apples");

    let result: String = print_hello(apples);
    println!("Hello 2, {}!", result);
}
fn print_hello(value: String) -> String {
    println!("Hello, {}!", value);
    return value;
}
