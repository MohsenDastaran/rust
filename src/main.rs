fn main() {
    // stack data
    let my_stack_value: i32 = 2;
    let my_intger_refrence: &i32 = &my_stack_value; // borrow with &
    println!("{my_intger_refrence}");

    // ////////////////////////////////////////////// //

    // heap data
    let my_heap_value: String = String::from("value");
    let my_heap_refrence: &String = &my_heap_value;

    println!("{my_heap_refrence}");
}
