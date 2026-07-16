fn main() {
    let burger: String = String::from("burger");
    add_fries(burger);
}
fn add_fries(mut meal: String) {
    meal.push_str(" with fries");
    println!("Meal: {}", meal);
}
