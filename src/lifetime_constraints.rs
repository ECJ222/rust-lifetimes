// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

// Declare the Movie struct with a title and a rating
struct Movie<'a> {
    title: &'a str,
    rating: u8,
}

// Declare the Reviewer struct with a reference to a Movie and a name
struct Reviewer<'a> {
    movie: &'a Movie<'a>,
    name: &'a str,
}

impl<'a> Reviewer<'a> {
    // Print the review information
    fn print_review(&self) {
        println!(
            "{} rated the movie '{}' with a score of {}",
            self.name,
            self.movie.title,
            self.movie.rating
        );
    }
}

fn main() {
    // Create a movie instance
    let movie = Movie {
        title: "The Rust Movie",
        rating: 8,
    };

    // Create a reviewer instance with a reference to the movie
    let reviewer = Reviewer {
        movie: &movie,
        name: "Alice",
    };

    // Print the review information
    reviewer.print_review();
}

// Output: Alice rated the movie 'The Rust Movie' with a score of 8
