// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

fn shortest_route<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b { b } else { a }
}

fn main() {
    let a = 300000;
    let b = 100000;
    println!("{}km", shortest_route(&a, &b));
}

// Output: 100000km
