// Topic: Looping using the while statement
// 
// Program requirements:
// * Count down from 5 to 1, displays the countdown
// in terminal then prints done when it complete.
// 
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within thee while loop
// * Do not use break to exit the while loop

fn main(){
    let mut count_down = 5;
    while count_down >= 1{
        println!("{count_down}");
        count_down -= 1;
    }
    println!("Done!");
}