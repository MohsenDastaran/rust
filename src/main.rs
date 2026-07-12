fn is_even(num: &i32) -> &str {
    // return num % 2 == 0;

    // with if
    if num % 2 == 0 { "true" } else { "false" }
}

fn main() {
    println!("{}", is_even(&2));
    println!("{}", is_even(&3));
    println!("{}", is_even(&4));
    println!("{}", is_even(&5));
}
