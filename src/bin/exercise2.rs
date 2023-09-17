// Topic: Basic arithmetic
// 
// Program requirements:
// * Display the results of two numbers
// 
// Notes:
// * Use the funtion to add two numbers together
// * Use the function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn  add(x:i32, y:i32)->i32{
    x + y
}
fn result(){
    println!("{:?}", add(2,4));
}
fn main(){
    result()
}