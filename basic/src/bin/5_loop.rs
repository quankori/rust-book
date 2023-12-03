// Topic: Looping using loop statement
//
// Requirements:
// * Display "1" through "4" using a counter variable
//
// Notes:
// * Use a mutable integer variable set to 1
// * Use a loop statement
// * Print the counter variable within the loop statement
// * Use break to exit the loop

fn main() {
    // * Use a mutable integer variable set to 1
    let mut counter = 1;

    // * Use a loop statement
    loop {
        // * Print the counter variable within the loop statement
        println!("{}", counter);
        // * Use break to exit the loop
        if counter == 4 {
            break;
        }
        counter += 1;
    }

    // * Use a while statement
    let mut counter_2 = 1;
    while counter_2 <= 4 {
        println!("{}", counter_2);
        counter_2 += 1;
    }
}
