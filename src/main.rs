fn main() {
    let is_concert = true;
    let is_event = is_concert;
    println!("Is it a concert? {}", is_concert);

    // /////////////////////////////////////// //

    // let sushi = "Salmon";
    // let dinner = sushi;
    // println!("What is for dinner? {}", sushi);
    // println!("What is for dinner? {}", dinner);

    // /////////////////////////////////////// //

    let sushi: String = String::from("Salmon");
    let dinner: String = sushi;
    // println!("What is for dinner? {}", sushi);
    println!("What is for dinner? {}", dinner);

    // /////////////////////////////////////// //

    let meal: String = String::from("Salmon");
    let cloned_meal: String = meal.clone();
    let cleared: String = eat_meal(meal);
    println!("old: {} , new: {}", cloned_meal, cleared);
}

fn eat_meal(mut s: String) -> String {
    s.clear();
    s
}
