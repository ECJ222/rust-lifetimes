// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

trait Summary<'a> {
    fn summarize(&'a self) -> String;
}

struct NewsArticle<'a> {
    headline: &'a str,
    location: &'a str,
    author: &'a str,
    content: &'a str,
}

impl<'a> Summary<'a> for NewsArticle<'a> {
    fn summarize(&'a self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    let article = NewsArticle {
        headline: "A brand new world",
        location: "New York",
        author: "John Doe",
        content: "This is the content of the article",
    };

    println!("{}", article.summarize());
}

// Output: A brand new world by John Doe (New York)
