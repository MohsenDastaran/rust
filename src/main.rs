fn main() {
    let mut current_meal: String = String::new();
    println!("What is your meal? {}", current_meal);
    add_flour(&mut current_meal);
    println!("What is your meal? {}", current_meal);

    fn add_flour(meal: &mut String) {
        meal.push_str(" with flour")
    }
}
// we do this to avoid
// unnecessary return of the String

// type options:
// meal: String : regular string
// mut meal: String : mutable string
// meal: &String : reference to a string
//  meal: &mut String : mutable reference to a string
