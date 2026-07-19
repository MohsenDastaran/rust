enum Milk {
    Lowfat(i32),
    Whole,
    NonDiary { kind: String },
}

fn main() {
    let my_beverage = Milk::Whole;
    if let Milk::Whole = my_beverage {
        println!("fdgdfgdfg")
    }

    let my_beverage2 = Milk::Lowfat(12);

    if let Milk::Lowfat(percent) = my_beverage2 {
        println!("fdgdfgdfg {percent}")
    }

    let my_beverage3 = Milk::NonDiary {
        kind: "Hahaaaaaa".into(),
    };

    if let Milk::NonDiary { kind } = my_beverage3 {
        println!("Kind: {kind}")
    }
}
