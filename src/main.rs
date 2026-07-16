fn main() {
    let current_meal: String = String::new();
    let fl: String = add_flour(current_meal);
    let su: String = add_sugar(fl);

    println!("{}! ", su);
}

fn add_flour(mut meal: String) -> String {
    meal.push_str(" with flour");
    meal
}

fn add_sugar(mut meal: String) -> String {
    meal.push_str(" with sugar");
    meal
}

//  HOW TO AVOID RETURNING THE VALUE OF A VARIABLE IN RUST:

// Next Section is about that: Refrences
