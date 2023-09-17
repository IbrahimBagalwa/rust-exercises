// Topic: Basic match expression
// 
// program requirements:
// * Display "its true" or "its false" based on the value of the boolean variable
// 
// Notes:
// * Use the variable set either the valuee is true or false
// * Use match expressions to determine which message to display

fn main(){
    let is_boolean_value = true;
    match is_boolean_value{
        true => println!("Its true"),
        false =>println!("Its false"),
    }
}