use std::collections::HashMap;

#[derive(Debug)]
struct Contents{
    content: String,
}
fn main(){
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents { content: "stuff".to_owned() });
    lockers.insert(2, Contents { content: "short".to_owned() });
    lockers.insert(3, Contents { content: "gym short".to_owned() });
    lockers.insert(4, Contents { content: "gym".to_owned() });
    lockers.insert(5, Contents { content: "gym stuff".to_owned() });

    for (lockers_number, content) in lockers.iter() {
        println!("lockers_number: {:?} and content: {:?}", lockers_number, content);
    }
}