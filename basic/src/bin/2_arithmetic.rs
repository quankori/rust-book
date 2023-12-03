// Topic: Basic Arithmetic
//
// Requirements:
// * Display the results of sum of two numbers
//
// Notes:
// * Use a function to sum two numbers
// * Use a function to display the result

// * Use a function to sum two numbers
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// * Use a function to display the result
fn display(a: i32, b: i32, sum: i32) {
    println!("{} + {} = {}", a, b, sum);
}

fn main() {
    let a = 10;
    let b = 20;
    let sum = add(a, b);
    display(a, b, sum)
}
