use std::collections::{HashMap, hash_map};

fn main() {
    let my_vetor = vec![5, 12, 500, 214, 5];
    let mut my_iterable = my_vetor.into_iter(); // make vector iterable

    for number in my_iterable {
        println!("{:?}", number);
    }
    // println!("{:?}", my_iterable); // for loop, borrowed it.

    println!("------------------------------------------------------------------",);

    let my_vetor_2 = vec![5, 12, 500, 214, 5];

    // but this will work too, why?
    // because rust will turn it to iterable for for loop
    for number in my_vetor_2 {
        println!("{:?}", number);
    }

    // bool
    // let my_vetor_bool: Vec<bool> = vec![true, false, true];
    // let my_iterable_bool = my_vetor_bool.into_iter(); // make vector iterable

    // //Hashmap
    // let mut my_hashmap: HashMap<&str, i32> = HashMap::new();
    // my_hashmap.insert("CBS", 2);
    // my_hashmap.insert("Intl", 3);
    // my_hashmap.insert("BBC", 4);

    // println!("{:?}", my_hashmap);
    // let my_iterable_hashmap: hash_map::IntoIter<&str, i32> = my_hashmap.into_iter(); // make vector iterable

    // println!("{:?}", my_iterable_hashmap);
}
