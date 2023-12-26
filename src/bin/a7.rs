// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
    Transparent,
}

fn color_name(colors: Colors) {
    match colors {
        Colors::Red => println!("red"),
        Colors::Green => println!("green"),
        Colors::Blue => println!("blue"),
        Colors::Transparent => println!("transparent"),
    }
}

fn main() {
    color_name(Colors::Red);
    color_name(Colors::Blue);
    color_name(Colors::Green);
    color_name(Colors::Transparent);
}
