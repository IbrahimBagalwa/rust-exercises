// Topic: HashMap
// 
// Requirements:
// * Print the name and number of items in stock for furniture store
// * If the number of items is 0, print out "out of stock" instead of "out of stock 0
// * The store has :
//  * 5 Chairs
//  * 3 Beds
//  * 2 Tables
//  * 0 Couches
// * Print the total number of items in the store
//  
// Notes: 
// * use hashMap for the furniture store stock

use std::collections::HashMap;

fn main(){
    let mut store = HashMap::new();
        store.insert("Chairs", 5);
        store.insert("Beds",3);
        store.insert("Tables", 2);
        store.insert("Couches", 0);

    let mut total = 0;
    for (item, qty) in store.iter(){
        total = total + qty;
        let stock_count = if qty == &0 {
            "Out of stock".to_owned()
        }else{
            format!("{:?}", qty)
        };
        println!("item = {:?}, stock = {:?}", item, stock_count);
    }
    println!("total stock = {:?}", total);
}