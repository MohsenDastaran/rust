fn main() {
    /*
    String - A dynamic piece of text stored on the heap
    at runtime

    &String ("ref String") - A reference to a heap String

    str - A hardcoded, read-only piece of text encoded in
    the binary

    &str ("ref str") - A reference to the text in the memory
    that has loaded the binary file
    */

    // String in heap data
    let my_heap_value: String = String::from("value");
    // String reference in heap data
    let my_heap_refrence: &String = &my_heap_value;

    // A reference to text stored in the program's read-only memory.
    let my_str: &str = "My Text";

    // Why is this &str and not str?
    // Because `str` is an unsized type and cannot be stored directly in a variable.
    // `my_str` is a reference to the string literal "My Text", which is stored
    // in the program's binary (read-only memory).
    println!("{my_heap_refrence} {my_str}");
}
