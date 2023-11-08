// Topic: Ownership
// 
// Requiements:
// * Print out quantity and id number of a grocery item;
// 
// Notes: 
// * Use struct for a Grocery item
// * Use two i32 fields for the quantity and id number
// * create a function to display the quantity
// * create a function to display the id number

struct GroceryItem{
    quantity: i32,
    id: i32,
}

fn display_quantity(grocery_item: &GroceryItem){
    println!("quantity: {:?}", grocery_item.quantity);
}

fn display_id_number(grocery_item: &GroceryItem){
    println!("IDNumber: {:?}", grocery_item.id);
}

fn main(){
    let grocery_item = GroceryItem{
        quantity: 10,
        id: 100,
    };
    display_quantity(&grocery_item);
    display_id_number(&grocery_item);
}