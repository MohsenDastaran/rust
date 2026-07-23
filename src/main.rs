fn main() {
    let my_str = "value🍵";

    // loop over each byte
    for byte in my_str.bytes() {
        println!("{byte}")
    }
    // loop over each char

    for ch in my_str.chars() {
        println!("{ch}")
    }
}
