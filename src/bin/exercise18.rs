// Topic: Result
// 
// Requirements:
// * create the structure name `Adult` that represents a person aged 21 or older:
//      * the structure must contain the person's name and age
//      * Implement Debug print functionality using `derive` 
// * Implement a `new` function for the `Adult` structure that returns a result:
//      * The Ok variant should contain the initialized structure, but only if the person is aged 21 or older
//      * The Err variant should contain a string (or &str) that explains wht the structure could not be created
// * Instantiate two `Adult` structures:
//      * one should be aged under 21
//      * one should be 21 or over
// * Use `match` to print out a message for each `Adult`
//      * for the Ok variant, print any message you want
//      * for the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                name: name.to_string(),
                age,
            })
        } else {
            Err("Age must be at least 21")
        }
    }
}


fn main () {
    let child = Adult::new("Aldo", 16);
    let adult = Adult::new("Sandrine", 21);
    match child {
        Ok(c) => println!("{} is old years {} ", c.name, c.age),
        Err(e) => println!("Error occured : {}", e),
    }

     match adult {
        Ok(c) => println!("{} is old years {} ", c.name, c.age),
        Err(e) => println!("Error occured : {}", e),
    }
}