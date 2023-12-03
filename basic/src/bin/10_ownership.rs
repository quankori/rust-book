// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of an grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id
// * Create a function to display the quantity
// * Create a function to display the id

// * Use a struct for the grocery item
struct GroceryItem {
    quantity: i32,
    id: i32,
}

// * Create a function to display the quantity
fn display_quantity(item: &GroceryItem) {
    println!("Quantity: {}", item.quantity);
}

// * Create a function to display the id
fn display_id(item: &GroceryItem) {
    println!("ID: {}", item.id);
}

fn main() {
    // * Use two i32 fields for the quantity and id
    let item = GroceryItem {
        quantity: 1,
        id: 100,
    };

    display_quantity(&item);
    display_id(&item);
}
