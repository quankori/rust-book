// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of some drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of a drink
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink information
// * Use a match expression to determine what message to print

// * Use an enum to create different flavors of a drink
enum Flavor {
    Orange,
    Grape,
}

// * Use a struct to store drink flavor and fluid ounce information
struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink(drink: Drink) {
    // * Use a match expression to determine what message to print
    match drink.flavor {
        Flavor::Orange => println!("Orange"),
        Flavor::Grape => println!("Grape"),
        Flavor::Cherry => println!("Cherry"),
    }
    println!("{} oz", drink.fluid_oz);
}

fn main() {
    // * Print the flavor of some drink and it's fluid ounces
    let drink = Drink {
        flavor: Flavor::Orange,
        fluid_oz: 12.0,
    };
    print_drink(drink);
    let drink = Drink {
        flavor: Flavor::Grape,
        fluid_oz: 5.0,
    };
    print_drink(drink);
}
