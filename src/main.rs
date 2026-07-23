use std::collections::HashMap;

fn main() {
    let mut my_hashmap: HashMap<&str, i32> = HashMap::new();
    my_hashmap.insert("CBS", 2);
    my_hashmap.insert("Intl", 3);
    my_hashmap.insert("BBC", 4);

    for (ch, status) in &mut my_hashmap {
        // the order is different each time
        println!("Channel: {:?} Complete: {:?}", ch, status);
        *status = 256;
    }
    println!("Channels: {:?}", my_hashmap)
}
