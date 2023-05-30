// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

fn main() {
    let mut vec = vec![10, 11];
    let first = &mut vec[0];
    *first = 6;
    println!("{:?}", vec);
}

// Output: [6, 11]
