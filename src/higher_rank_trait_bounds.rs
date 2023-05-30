// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

// Define a trait with a method that takes a closure with a reference parameter.
trait RefProcessor {
    fn process_refs<F>(&self, f: F) where F: Fn(&i32);
}

// Implement the RefProcessor trait for a Vec<i32>.
impl RefProcessor for Vec<i32> {
    fn process_refs<F>(&self, f: F) where F: Fn(&i32) {
        for item in self {
            f(item);
        }
    }
}

// A function that takes a type implementing RefProcessor and a closure with a generic lifetime.
fn process_all_items<T>(ref_processor: &T, closure: impl for<'a> Fn(&'a i32)) where T: RefProcessor {
    ref_processor.process_refs(closure);
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Pass a closure that prints the square of each item.
    process_all_items(&numbers, |x| println!("{}", x * x));
}

// Output: 1
//         4
//         9
//         16
//         25
