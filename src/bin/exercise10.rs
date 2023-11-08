// Topic: Working with expressions
// 
// Requirements: 
// * Print "its big" if the variable is > 100
// * Print "its small" if the variable is <=100
// 
// Notes:
// * Use a boolean variable set to an expression
//    that can be determines whether the variable is > 100 or <= 100
// * Use the funtion to print the messages
// * Use match expression to determine which messages to print

fn print_message(is_value_big: bool){
    match is_value_big{
        true => println!("its big"),
        false => println!("its small"),
    };
}
fn main(){
    let value = 300;
    let is_value_big =  value > 100;
    print_message(is_value_big);
}