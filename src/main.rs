use std::{char, collections::HashMap};

fn count_charachters(text: &str) -> HashMap<char, u16> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();
    // for word in words {
    //     for character in word.chars() {
    //         let count = counts.entry(character).or_insert(0);
    //         *count += 1;
    //     }
    // }
    // counts

    // using for_each
    words.for_each(|word| {
        word.chars().for_each(|character| {
            let count: &mut u16 = counts.entry(character).or_insert(0);
            *count += 1;
        });
    });
    counts
}

fn main() {
    println!(
        "{:?}",
        count_charachters("sdfsdcvs  ds ds sd sd d sd sdf sd sd sd sd sd sd sd sd ")
    )
}
