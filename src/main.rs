fn main() {
    #[derive(Debug)]
    struct Cofee {
        name: String,
        price: f64,
        is_hot: bool,
    }

    let mut mocha: Cofee = Cofee {
        name: String::from("Mocha"),
        price: 3.5,
        is_hot: true,
    };

    mocha.name = String::from("Cappuccino");
    println!("{:#?}", mocha);
}
