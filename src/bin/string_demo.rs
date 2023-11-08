struct LineItem{
    name: String,
    count: i32,
}

fn print_name(name:&str){
    println!("name: {:?}", name);
}
fn print_count(count:i32){
    println!("count: {:?}", count);
}

fn main(){
    let my_line_items = vec![
        LineItem{name: "milk".to_string(), count: 10},
        LineItem{name: "eggs".to_string(), count: 20},
        LineItem{name: "bread".to_string(), count: 30},
    ];
    for line_item in &my_line_items {
        print_name(&line_item.name);
        print_count(line_item.count);
    }
}