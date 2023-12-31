// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Person {
    age: u8,
    name: String,
    fav_color: Color,
}

fn print_name(name: String) {
    println!("{name:?}");
}

fn print_color(color: Color) {
    println!("{color:?}");
}

fn main() {
    let persons = vec![
        Person {
            name: String::from("Alice"),
            age: 30,
            fav_color: Color::Red,
        },
        Person {
            name: String::from("Zappy"),
            age: 26,
            fav_color: Color::Blue,
        },
        Person {
            name: "Alice".to_string(),
            age: 15,
            fav_color: Color::Green,
        },
    ];

    for person in persons {
        if person.age > 18 {
            print_name(person.name);
            print_color(person.fav_color);
        }
    }
}
