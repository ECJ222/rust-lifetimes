// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

fn main() {
    let vec = vec![10, 11];
    for i in &vec {
        println!("{}", i);
    }
}

// Output: 10
//         11
