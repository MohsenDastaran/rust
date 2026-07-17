struct Cofee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let cofee: Cofee = make_coffee("name".into(), 3.5, true);
    println!("{}", cofee.name);
}
fn make_coffee(name: String, price: f64, is_hot: bool) -> Cofee {
    Cofee {
        name,
        price,
        is_hot,
    }
}
