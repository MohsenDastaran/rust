fn main() {
    let ok: Result<i32, String> = Result::Ok(5);
    let disaster: Result<i32, &str> = Result::Err("Error");
    println!("{ok:?}");
    println!("{disaster:?}");
}
