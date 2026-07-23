use std::collections::HashSet;

fn main() {
    let numbers: Vec<i32> = vec![
        1, 5, 4, 54, 54, 54, 654, 65, 132, 132, 132, 1321, 3651, 589, 7984, 854, 354,
    ];

    // using Vec<i32>

    // let squares = numbers
    //     .iter()
    //     .map(|num: &i32| num.pow(2))
    //     .collect::<Vec<i32>>();

    // using HashSet<i32>
    let squares = numbers
        .iter()
        .map(|num: &i32| num.pow(2))
        .collect::<HashSet<i32>>();
    println!("{squares:?}");
}
