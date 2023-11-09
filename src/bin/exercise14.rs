// Topic : Strings
// 
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
// 
// Notes:
// * Use a struct for a persons, name, age and favorite color
// * The color and name should be stored as a string
// * Create and store at least 3 people in the vector
// * Iterate through vector using a for...in loop
// * Use an if expression to determine which person's info should be printed
// * The name and color should be printed using a function 

struct Person {
    name: String,
    age: i32,
    favorite_color: String,
}

fn print_name(name: &str){
    println!("name: {:?}", name);
}

fn print_color(color: &str){
    println!("color: {:?}", color);
}

fn main(){
    let my_people = vec![
        Person{name: "Ivad".to_owned(), age: 13, favorite_color: String::from("Black")},
        Person{name: "Izzedin".to_owned(), age: 23, favorite_color: String::from("Blue")},
        Person{name: "Ibrahim".to_owned(), age:10, favorite_color: String::from("black")},
        Person{name: "Olivier".to_owned(), age: 6, favorite_color: String::from("Black")},
        Person{name: "Fabien".to_owned(), age: 9, favorite_color: String::from("Blue")},
        Person{name: "Eric".to_owned(), age:5, favorite_color: String::from("black")}
    ];
    for person in &my_people{
        if person.age < 10 {
            print_name(&person.name);
            print_color(&person.favorite_color);
        }
    }
}