fn main() {
    let age: i8 = 29;
    let is_handsome: bool = true;

    println!("{age}");
    println!("{is_handsome}");
    // both exist here
}

// after scope, because of LIFO, first, is_handsome will be removed & after that, age.
