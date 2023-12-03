// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of cartesian coordinates is
//   greater than 5, less than 5, or equal to 5.
//
// Notes:
// * Use a function that returns a tuple
// * Descructure the return value into two variables
// * Use an if..else if..else block to print the correct message

// * Use a function that returns a tuple
fn coordinates() -> (i32, i32) {
    (2, 5)
}

fn main() {
    // * Descructure the return value into two variables
    let (x, y) = coordinates();
    // * Use an if..else if..else block to print the correct message
    if y > 5 {
        println!("Greater than 5");
    } else if y < 5 {
        println!("Less than 5");
    } else {
        println!("Equal to 5");
    }
}
