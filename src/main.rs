fn main() {
    struct Cofee {
        name: String,
        price: f64,
        is_hot: bool,
    }

    let mocha: Cofee = Cofee {
        name: String::from("Mocha"),
        price: 3.5,
        is_hot: true,
    };
    // println!("{:#?}", mocha);
}
