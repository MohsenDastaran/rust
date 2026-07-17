fn main() {
    let  car: String = String::from("Red");
    let ref1: &String = &car; // reference to car
    let ref2: &String = &car; // another reference to car


    println!("ref1: {}, ref2: {}", ref1, ref2);
}

