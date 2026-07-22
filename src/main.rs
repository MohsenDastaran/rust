fn main() {
    let mut animals = vec!["zebra", "monkey", "leopard"];
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals));
    println!("{:?}", length_of_last_element(&mut animals))
}

fn length_of_last_element(input: &mut Vec<&str>) -> Option<usize> {
    let last = input.pop()?;
    Some(last.len())
}
