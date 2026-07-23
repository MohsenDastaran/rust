use std::collections::{HashMap, hash_map};

fn main() {
    let my_vetor = vec![5, 5, 5, 214, 5];
    let my_iterable = my_vetor.into_iter(); // make vector iterable

    // bool
    let my_vetor_bool: Vec<bool> = vec![true, false, true];
    let my_iterable_bool = my_vetor_bool.into_iter(); // make vector iterable

    //Hashmap
    let mut my_hashmap: HashMap<&str, i32> = HashMap::new();
    my_hashmap.insert("CBS", 2);

    let my_iterable_hashmap: hash_map::IntoIter<&str, i32> = my_hashmap.into_iter();
}
