// If you want to run just copy and paste the code into main.rs and run `cargo run` in your terminal.

struct Path<'a> {
    point_x: &'a i32,
    point_y: &'a i32,
}

fn main() {
    let p_x = 3200;
    let p_y = (p_x / 2) as i32;
    let maze = Path { point_x: &p_x, point_y: &p_y };
    println!("x = {}, y = {}", maze.point_x, maze.point_y);
}

// Output: x = 3200, y = 1600
