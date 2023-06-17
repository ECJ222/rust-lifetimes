// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

// lifetime_constraints.rs

// Declare the Movie struct with a title and a rating
#[derive(Debug)]
struct Movie<'a> {
    title: &'a str,
    rating: u8,
}

// Declare the Reviewer struct with a reference to a Movie and a name
#[derive(Debug)]
struct Reviewer<'a, 'b: 'a> {
    movie: &'a Movie<'b>,
    name: &'a str,
}

impl<'a, 'b> Reviewer<'a, 'b> {
    // Create a new review
    fn new(name: &'a str, movie: &'b Movie) -> Self {
        Reviewer { movie, name }
    }
}

fn main() {
    // Create a movie instance
    let movie = Movie {
        title: "The Rust Movie",
        rating: 8,
    };

    // Print the review information
    println!("{:?}", Reviewer::new("Alice", &movie));
}

// Output: Reviewer { movie: Movie { title: "The Rust Movie", rating: 8 }, name: "Alice" }
