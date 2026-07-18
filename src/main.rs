#[derive(Debug)]
struct Cofee {
    name: String,
    price: f64,
    is_hot: bool,
}

fn main() {
    let cofee: Cofee = make_coffee("name".into(), 3.5, true);
    println!("{}", cofee.name);

    //  new cofee with ..
    let mut caramel_macchiato = Cofee {
        name: String::from("Caramel Macchiato"),
        ..cofee // it should be on last line of the struct
    };

    println!("{:?}", caramel_macchiato);
    println!("{}", caramel_macchiato.name);

    // update the name of the coffee
    drink_cofee(&mut caramel_macchiato);
}

fn make_coffee(name: String, price: f64, is_hot: bool) -> Cofee {
    Cofee {
        name,
        price,
        is_hot,
    }
}
fn drink_cofee(coffee: &mut Cofee) {
    println!("D {}", coffee.name); // works only with &Cofee in parameter , we cant edit
    coffee.name = String::from("Cappuccino");
    println!("D {}", coffee.name); // works only with &mut Cofee in parameter , we can edit
}
