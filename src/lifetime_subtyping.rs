// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

fn lifetime_subtyping<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = lifetime_subtyping(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// Output: The longest string is abcd
