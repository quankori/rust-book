// Topic: Working with Enums
//
// Requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as the variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color is passed in and print the correct message

// * Use an enum with color names as the variants
enum Color {
    Red,
    Green,
    Blue,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color is passed in and print the correct message
fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}

fn main() {
    // * Prints the name of a color to the terminal
    print_color(Color::Red);
    print_color(Color::Green);
    print_color(Color::Blue);
}
