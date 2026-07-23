use std::{collections::HashSet, mem::replace};

fn main() {
    let names = [
        String::from("Mohsen"),
        String::from("Ali"),
        String::from("Pouya"),
    ];

    let name_lengths = names
        .iter()
        .map(|name| name.to_lowercase())
        .map(|name| name.replace("i", "@@"))
        .map(|name| name.len())
        .collect::<Vec<usize>>();

    println!("{:?}", name_lengths)
}
