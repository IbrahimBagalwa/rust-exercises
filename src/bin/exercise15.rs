// Topic: Advanced match
// 
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip or Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
// 
// Notes:
// * Use an enum for the ticket with data associated with each variant
// * Create one of each ticket and place it in a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String, i32),
    Standard(i32),
    Vip(String, i32),
}

fn main() {
    let my_tickets: Vec<Ticket> = vec![
        Ticket::Backstage(String::from("John"), 10),
        Ticket::Vip(String::from("Jane"), 20),
        Ticket::Standard(30),
        Ticket::Backstage(String::from("Mary"), 40),
        Ticket::Standard(50),
    ];
    for ticket in &my_tickets {
        match ticket {
            Ticket::Backstage(name, price) => {
                println!("Backstage ticket for: {:?} costs {:?} dollars", name, price);
            }
            Ticket::Vip(name, price) => {
                println!("Vip ticket for: {:?} costs {:?} dollars", name, price);
            }
            Ticket::Standard(price) => {
                println!("Standard ticket costs {:?} dollars", price);
            }
        }
    }
}