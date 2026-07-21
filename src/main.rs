use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f32> = HashMap::new();
    menu.insert("Ghorme".into(), 19.23);
    menu.insert("Gheyme".into(), 19.00);
    menu.insert("Kabab".into(), 24.99);

    println!("Testing Main {:?}", menu);

    let mut country_capitals = HashMap::<&str, &str>::new(); // with turbo-fish operator => ::<&str, &str>::

    country_capitals.insert("Iran", "Tehran");
    country_capitals.insert("Germany", "Berlin");
    println!("Testing Main {:?}", country_capitals);
}
