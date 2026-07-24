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
}
