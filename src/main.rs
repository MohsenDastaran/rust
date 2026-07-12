
fn main() {
learn_lang("Rust");
learn_lang_via("Rust"     , "Course");
// 1 parameter
fn learn_lang(lang: &str) {
    println!("Im learning {lang}")
}


// 2 parameter 
fn learn_lang_via(lang: &str, via: &str) {
    println!("Im learning {lang} via {via}")
}


}
