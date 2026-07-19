fn main() {
    // let movies = ["Se7en", "Titanic"]; // array has fixed length
    // movies.push("Usual suspect")  // doesnt work

    // let vec_movies: Vec<String> = Vec::<String>::new();  //empty
    let vec_movies: Vec<String> = vec!["Se7en".into(), "Titanic".into()];
    println!("{:?}", vec_movies)
}
