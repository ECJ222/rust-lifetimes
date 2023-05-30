// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

// Lifetime bounds.
use std::fmt::Display;

#[derive(Debug)]
struct Movie {
    title: String,
    rating: f64,
}

impl Movie {
    fn new<T: Display + PartialOrd>(title: String, rating: T) -> Self {
        Movie {
            title,
            rating: rating.to_string().parse().unwrap(),
        }
    }
}

fn main() {
    let movie = Movie::new("The Shawshank Redemption".to_string(), 9.3);
    println!("{:#?}", movie);
}

// Output: Movie {
//              title: "The Shawshank Redemption",
//              rating: 9.3,
//         }
