// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

#[derive(Debug)]
struct Num {
    x: i32,
}

impl Num {
    fn compare<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.x > other.x { self } else { other }
    }
}

fn main() {
    let num = Num { x: 3 };
    let other_num = &num;
    println!("{:?}", num.compare(other_num));
}

// Output: Num { x: 3 }
