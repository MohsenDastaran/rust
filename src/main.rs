fn main() {
    let numbers: Vec<i32> = vec![
        1, 5, 4, 54, 54, 54, 654, 65, 132, 132, 132, 1321, 3651, 589, 7984, 854, 354,
    ];

    let squares = numbers.into_iter().map(|num: i32| num.pow(2));
    println!("{squares:?}"); // same as numbers

    for num in squares {
        println!("{num:?}"); // now, we can see the real values becaue of the laziness 
    }
}
