// Topic: implementing fonctionality with the impl keyword
// 
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weights and color
// 
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use a enum for the box colors
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColor {
    Red,
    Green,
    Blue,
}

impl BoxColor {
    fn print(&self){
        match self{
            BoxColor::Red => println!("Red"),
            BoxColor::Green => println!("Green"),
            BoxColor::Blue => println!("Blue"),
        }
    }
}

struct Dimension{
    weight: f64,
    height: f64,
    depth: f64,
}

impl Dimension{
    fn print(&self){
        println!("weight: {:?}", self.weight);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}
struct BoxCharacteristic{
    dimensions: Dimension,
    weight: f64,
    color: BoxColor,
}


impl BoxCharacteristic{
    fn new(dimensions: Dimension , weight: f64, color: BoxColor  ) -> Self{
        Self{
            dimensions,
            weight,
            color,
        }
    }
    fn print_characteristics(&self){
        self.color.print();
        self.dimensions.print();
        println!("Weight: {:?}", self.weight);
    }
}
fn main(){
    let small_dimensions = Dimension{
        weight: 1.0,
        height: 4.0,
        depth: 6.0,
    };
    let small_box = BoxCharacteristic::new(small_dimensions, 4.2, BoxColor::Red);
    small_box.print_characteristics();
}