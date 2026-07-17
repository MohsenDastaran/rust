fn main() {
    let city = create_city();
    println!("City: {}", city);
}

// dangling reference example, city will be dropped at the end of the function and the reference will be invalid
fn create_city() -> String {
    let city = String::from("Sari");
    city
}
