// Topic: Vector
// 
// Requirements:
// * Print 10, 20, "thirty" and 40 in a loop
// * Print the total number of elements in a vector
// 
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using for in loop
// * Determine whether to print a number or "thirty" inside the loop
// * Use the len() function to print the number of elements in a vector

fn main(){
    let my_vectors = vec![10, 20, 30, 40];
    for number in &my_vectors {
        match number{
            30 => println!("thirty"),
            _ => println!("{:?}", number),
        }
    }
    println!("Elements are = {:?}", my_vectors.len());
    
}
