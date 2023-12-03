// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is bigger than 100
// * Print "its small" if a variable is smaller than 100
//
// Notes:
// * Use a boolean variable to an expression that evaluates to true or false
// * Use a function to display the message
// * Use a match expression to determine what message to display

fn print_message(gt_100: bool) {
    // * Use a match expression to determine what message to display
    match gt_100 {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    // * Use a boolean variable to an expression that evaluates to true or false
    let value = 100;
    let is_gt_100 = value > 100;
    print_message(is_gt_100);
}
