use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<char, u16> {
    let words = text.split_whitespace();
    let mut counts = HashMap::new();
    // let mut letters = HashMap::new();
    for word in words {
        for character in word.chars() {
            let count = counts.entry(character).or_insert(0);
            *count += 1;
        }
        // let count = counts.entry(word).or_insert(0);
        // *count += 1
    }
    counts
}

fn main() {
    println!(
        "{:?}",
        count_words("sdfsdcvs  ds ds sd sd d sd sdf sd sd sd sd sd sd sd sd ")
    )
}
