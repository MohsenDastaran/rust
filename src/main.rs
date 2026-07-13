fn main() {
    let mut counter: i32 = 0;
    loop {
        counter += 1;

        let _ = counter <= 2 && continue; // continue if counter is less than or equal to 2

        println!("Hello, world! {} times", counter);

        if counter >= 5 {
            break;
        }

        //  or
        // let _ = counter >= 5 && break;
    }
}
