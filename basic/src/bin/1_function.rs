// Topic: Functions
//
// Requirements:
// * Display your first name and last name in separate functions.
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use a function to call the other two functions

// * Use a function to display your first name
fn first_name() {
    println!("My first name is: {}", "Nguyen");
}

// * Use a function to display your last name
fn last_name() {
    println!("My last name is: {}", "Quan");
}
fn main() {
    first_name();
    last_name();
}
