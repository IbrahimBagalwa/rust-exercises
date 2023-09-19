// Topic: Data management using Tuples
// Program requirements:
// * Print whether the y_value of cartesian coordinate is grater than 5, less than 5 or equal to 5
// 
// Notes: 
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use if...else if...else block to determine what to print

fn cartesian_coordinate()->(i32, i32){
    (2,3)
}
fn main(){
    let (y_value, x_value) = cartesian_coordinate();
    if y_value > 5{
        println!("Y value is greater than 5");
    }else if y_value < 5{
        println!("Y value is less to 5")
    }else{
        println!("Y value is equal than 5");
    }
}
