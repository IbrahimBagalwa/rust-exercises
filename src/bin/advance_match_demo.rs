enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main(){
    let n = 4;
    match n {
        3 => println!("We have: {:?}",n),
        other => println!("{:?}", other)
    }
    let flat = Discount::Flat(2);
    match flat{
        Discount::Flat(2) => println!("Flat discount: 2"),
        Discount::Flat(amount) => println!("Percent discount of: {:?}", amount),
        _ => (),
    }

    let ticket = Ticket {
        event: String::from("Rust"),
        price: 50,
    };
    match ticket {
        Ticket { price: 50, event } => println!("We have a Rust ticket"),
        Ticket {price, ..} => println!("When we are matchin a struct the two dots means ignore other fields"),
    }
}