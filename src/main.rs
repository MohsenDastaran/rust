#[derive(Debug)]
enum Beans {
    Black,
    Pinto,
}


#[derive(Debug)]
enum Meat {
    Chicken,
    Beaf,
}

#[derive(Debug)]

enum ResaurantItem {
    Burrito(Meat),
    Bowl {meat: Meat, beans: Beans},
    Vegan,
}

fn main() {
    let lunch = ResaurantItem::Burrito(Meat::Chicken);
    let dinner = ResaurantItem::Burrito(Meat::Beaf);

    let brunch = ResaurantItem::Bowl {meat: Meat::Beaf, beans: Beans::Black };

    println!("{:?}, {:?}, {brunch:?}", lunch, dinner)
}
