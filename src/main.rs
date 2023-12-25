#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };

    // This will not compile because Point does not implement the Display trait
    // println!("Point is {}", point);

    // This will compile because Point implements the Debug trait
    println!("Point is {:?}", point);
}
