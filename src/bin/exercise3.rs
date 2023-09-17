// Topic: Control flow if ... else
// Program requirements:
// 
// * Display a message based on the value of a boolean variable
// * When the variable is set to true, display "Hello"
// * When the variable is set to false, display "Goodbye"
// 
// Notes:
// * Use a variable set either to true or false
// * Use an if..elsee block to determine which message to display
// * Use the println macro to display the message to the terminal

fn main(){
    let is_geeting = true;
    if is_geeting {
        println!("Hello");
    }else{
        println!("Goodbye");
    }
}