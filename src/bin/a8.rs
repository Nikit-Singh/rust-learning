// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

#[derive(Debug)]
enum Flavours {
    Sweet,
    Fruity,
    Sparkling,
}

struct Drink {
    flavour: Flavours,
    weight: i32,
}

fn print_details(drink: Drink) {
    match drink.flavour {
        Flavours::Sweet => println!("sweet"),
        Flavours::Fruity => println!("fruity"),
        Flavours::Sparkling => println!("sparkling"),
    }

    println!("oz {:?}", drink.weight);
}

fn main() {
    let drink = Drink{
        flavour: Flavours::Sweet,
        weight: 30,
    };
    print_details(drink);
}
