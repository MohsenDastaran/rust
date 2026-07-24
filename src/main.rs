fn main() {
    let nums = [
        10, 54, 0, 1, 0, 212, 54, 3541, 25341, 321, 321321, 3212, 121, 547, 8564, 8785, 465, 4654,
        65413, 241564, 251, 541,
    ];

    //filter
    let even_nums: Vec<i32> = nums.iter().filter(|num| *num % 2 == 0).copied().collect();
    println!("{:?}", even_nums);

    //find
    let first_odd = nums.iter().find(|num| *num % 2 != 0).copied();
    println!("{:?}", first_odd);
}
