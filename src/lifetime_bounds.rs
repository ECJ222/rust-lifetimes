// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

// Lifetime bounds.
use std::fmt::Display;

#[derive(Debug)]
struct Movie<'a, T> {
    title: &'a str,
    rating: T,
}

impl<'a, T: 'a + Display + PartialOrd> Movie<'a, T> {
    fn new(title: &'a str, rating: T) -> Self {
        Movie {
            title,
            rating,
        }
    }
}

fn main() {
    let movie = Movie::new("The Shawshank Redemption", 9.3);
    println!("{:#?}", movie);
}

// Output: Movie {
//              title: "The Shawshank Redemption",
//              rating: 9.3,
//         }
