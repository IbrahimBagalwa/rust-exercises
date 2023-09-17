// Topic: Control flow using if ... else if ...else
// Program requirements:
// Display ">5", "<5", or "=5" based on the value of the variable
// is > 5, < 5, or == 5, respectively
// 
// Notes:
// * Use a variable set to any integer value
// * Use an if...else if...else block to determine which messages to display
// * Use the println macro to display the message to the terminal

fn main(){
    let value = 4;
    if value > 5 {
        println!("grater than 5: {value}");
    }else if value < 5 {
        println!("less than 5: {value}");
    }else{
        println!("equal to 5: {value}");
    }
}