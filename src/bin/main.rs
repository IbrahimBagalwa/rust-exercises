// Match expression
// Match expression add logic to your program
// is similar to if..else the only difference is match must be exhaustive which means every possible option must be accounted for in your code
fn main() {
    let some_int = 4 ;
    match some_int{
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        4 => println!("its 4"),
        _ => println!("its something else"),
    }
}
fn match_expression(){
    let some_boolean = true;
    match some_boolean{
        true => println!("its true"),
        false => println!("its false"),
    }
}