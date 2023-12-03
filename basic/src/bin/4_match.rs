// Topic: Decision Making using match
//
// Requirements:
// * Display "one", "two", "three", or "other" based on the value of a variable of a variable set to 1, 2, 3, or anything else
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine what message to display
// * Use an underscore (_) to match anything else

fn main() {
    // * Use a variable set to any integer
    let my_int = 5;

    // * Use a match expression to determine what message to display
    match my_int {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
    // * Use an underscore (_) to match anything else
}
