// Topic: Decision making with match
// 
// Program requirements:
// * Display "one", "two", "three", or "Other" based on whether the value of a variable is 1,2,3 or some other number, respectively
// 
// Notes:
// * Use a variable set to any integer
// * Use match expression to determine which message to display
// * Use an underscore to match any other value

fn main(){
    let int_value = 1;
    match int_value {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}