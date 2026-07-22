use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f32> = HashMap::new();
    menu.insert("Ghorme".into(), 19.23);
    menu.insert("Gheyme".into(), 19.00);
    menu.insert("Kabab".into(), 24.99);

    let mut country_capitals = HashMap::<&str, &str>::new(); // with turbo-fish operator => ::<&str, &str>::

    country_capitals.insert("Iran", "Tehran");
    country_capitals.insert("Germany", "Berlin");

    println!("{}", menu["Ghorme"]); // the problem is if value doesnt exist, this will cause panic in runtime

    let value = country_capitals
        .get("Germafny") // see the type is Option, because it can be Some or None ,
        .copied() //use .copied() to remove 1 reference(&)
        .unwrap_or("Unknown Country"); // add default  

    println!("{:?}", value);
}
