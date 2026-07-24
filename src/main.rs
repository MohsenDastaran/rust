fn main() {
    let nums = [
        10, 54, 40, 1, 420, 212, 54, 3541, 25341, 321, 321321, 3212, 121, 547, 8564, 8785, 465,
        4654, 65413, 241564, 251, 541,
    ];

    //filter
    let even_nums: Vec<i32> = nums.iter().filter(|num| *num % 2 == 0).copied().collect();
    println!("{:?}", even_nums);

    //find
    let first_odd = nums.iter().find(|num| *num % 2 != 0).copied();
    println!("{:?}", first_odd);

    // lets assume we want to remove values more than 100 & make  the values *2
    // without filter_map
    let custom_values: Vec<i32> = nums
        .iter()
        .filter(|num| *num < &100)
        .copied()
        .map(|val| val * 2)
        .collect();
    println!("{:?}", custom_values);

    // with filter_map
    let custom_values2: Vec<i32> = nums
        .iter()
        .filter_map(|val| if *val > 100 { None } else { Some(*val * 2) })
        .collect();
    println!("{:?}", custom_values2);

    // when i want to use index, i can use enumerate
    let enumerate: Vec<_> = nums
        .iter()
        .enumerate()
        .filter(|(index, num)| *index > 15)
        .collect();
    println!("enumerate: {:?}", enumerate);

    // partition method: its like filter, but it gives removed values too(in a different data).

    //filter
    let even_nums_part: (Vec<i32>, Vec<i32>) = nums.into_iter().partition(|num| *num % 2 == 0);
    println!("Partition:  {:?}", even_nums_part);
}
