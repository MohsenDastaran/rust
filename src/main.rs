fn main() {
    println!("{}", add_5(5));
    identity::<i32>(5);
    identity::<&str>("sdfsdvc");
    identity(true);

    identity2("sdcsdc", 36.56)
}
// normal parameter type
fn add_5(value: i32) -> i32 {
    value + 5
}

//  but here, i want a function to have dynamic value
fn identity<T: std::fmt::Debug>(value: T) {
    println!("{:?}", value)
}
fn identity2<T: std::fmt::Debug, U: std::fmt::Debug>(value: T, another: U) {
    println!("{:?}, {:?}", value, another)
}
