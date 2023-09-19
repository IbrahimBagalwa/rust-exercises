// Topic Organizing similar data using structs
// 
// Program requirements:
// * Print the flavor of a drink and it's fluid ounces
// 
// Notes:
// * Use an Enum to create diffrent flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor ounces
// * Use match expression to print the drink flavor

enum Flavor {
    Sparkling,
    Sweet,
    Fruity
}

struct Drink{
    flavor: Flavor,
    fluid_ounces:f64,
}

fn print_drink_flavor(drink:Drink){
   match drink.flavor{
    Flavor::Sparkling => println!("Sparkling drink flavor"),
    Flavor::Sweet => println!("Sweet drink flavor"),
    Flavor::Fruity => println!("Fruity drink flavor"),
    _ => println!("Comming sun"),

   }
    println!("{}", drink.fluid_ounces);
}

fn main(){
    let speakling = Drink{
        flavor:Flavor::Sparkling,
        fluid_ounces: 23.0,
    };
    print_drink_flavor(speakling);
}