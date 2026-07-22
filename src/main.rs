use std::collections::HashMap;

fn main() {
    let mut menu: HashMap<String, f32> = HashMap::new();
    menu.insert("Ghorme".into(), 19.23);
    menu.insert("Gheyme".into(), 19.00);
    menu.insert("Kabab".into(), 24.99);

    println!("{}", menu["Ghorme"]);

    menu.insert("Ghorme".into(), 11.00);
    println!("{}", menu["Ghorme"]); // see the value is replaced
}
