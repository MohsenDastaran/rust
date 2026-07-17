fn main() {
    let values: [i32; 4] = [7452, 1234, 5678, 9101];
    let my_slice: &[i32] = &values[..2];
    println!("Last Name: {:?} {:?}", my_slice, values);
}
