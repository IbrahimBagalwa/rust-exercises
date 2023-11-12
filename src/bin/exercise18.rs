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
    age: i32
}
