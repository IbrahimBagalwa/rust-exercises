// Topic:  Browsing standard library documentation
// 
// Requirements:
// * Print a string in lower case and upper case
// 
// Notes:
// * Utilize the standard library functionality to
//  transform the string to upper case and lower case
// * Use 'rustup doc' in terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for the functionality to transform the string to upper and lower case
// * Trying to search for: to_uppercase and to_lowercase

fn main(){
    let my_string = String::from("here is my string");
    let my_string_upper = my_string.to_uppercase();
    let my_string_lower = my_string.to_lowercase();
    println!("my_string in upper case: {:?}", my_string_upper);
    println!("my_string in lower case: {:?}", my_string_lower);
}