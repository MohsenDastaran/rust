use std::collections::HashSet;

fn main() {
    let mut concert_queue: HashSet<&str> = HashSet::new(); // HashSet prevents duplicate value

    concert_queue.insert("value");
    concert_queue.insert("value");
    concert_queue.insert("value");
    println!("{:?}", concert_queue)
}
