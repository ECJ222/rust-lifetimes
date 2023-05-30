// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn new(part: &'a str) -> ImportantExcerpt<'a> {
        ImportantExcerpt { part }
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt::new(first_sentence);
    i.announce_and_return_part("Hello, world!");
}

// Output: Attention please: Hello, world!
