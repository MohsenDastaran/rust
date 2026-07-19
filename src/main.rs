enum Milk {
    Lowfat(i32),
    Whole,
    NonDiary { kind: String },
}

fn main() {
    let my_beverage = Milk::Whole;

    let Milk::Lowfat(percent) = my_beverage else {
        println!("There is no percent here");
        return;
    };

    println!("{}", percent)
}
