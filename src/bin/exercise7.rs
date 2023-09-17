// Topic: Working with enum
// 
// Program requirements:
// * Print the name of the color to the terminal
// 
// Notes:
// * Use an enum with color names as a variant
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use match expression to determine which color has to be printed

enum Color{
    Blue,
    Pink,
    Black,
    White
}

fn print_enum_color(color_name: Color){
    
    match color_name{
        Color::Blue => println!("The color is Blue"),
        Color::Pink => println!("The color is Pink"),
        Color::Black => println!("The color is Black"),
        Color::White => println!("The color is White"),
    }
}
fn main(){
    print_enum_color(Color::Blue);
}