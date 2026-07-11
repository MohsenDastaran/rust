
fn main() {
    //  1 to 30 (not 31)
let employee: (&str, i32, &str) = ("Mohsen", 30, "Dastaran");

println!("{} {}", employee.0 , employee.2);


let (first, age, last) = employee;

println!("{} {}", first , last );

}
