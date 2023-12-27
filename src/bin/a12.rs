// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

struct Dimensions {
    height: i32,
    width: i32,
    depth: i32,
}

struct Box {
    dimensions: Dimensions,
    weight: f32,
    color: Color,
}

impl Box {
    fn create() -> Self {
        Self {
            dimensions: Dimensions {
                height: 32,
                width: 10,
                depth: 5,
            },
            weight: 12.5,
            color: Color::Green,
        }
    }

    fn print(&self) {
        println!("Dimensions {:?}h {:?}w {:?}d", self.dimensions.height, self.dimensions.width, self.dimensions.depth);
        println!("Weight {:?}kg", self.weight);
        println!("Color {:?}", self.color);
    }
}

fn main() {
    let x = Box::create();
    x.print();
}
