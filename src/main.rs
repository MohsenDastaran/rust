fn main() {
    // let movies = ["Se7en", "Titanic"]; // array has fixed length
    // movies.push("Usual suspect")  // doesnt work

    // let vec_movies: Vec<String> = Vec::<String>::new();  //empty
    let mut vec_movies: Vec<String> = vec!["Se7en".into(), "Titanic".into()];
    println!("{:?}", vec_movies);

    vec_movies.push("Inception".into());
    vec_movies.push("Test".into());
    vec_movies.pop();
    println!("{:?}", vec_movies);

    vec_movies.remove(1);
    println!("{:?}", vec_movies)
}
