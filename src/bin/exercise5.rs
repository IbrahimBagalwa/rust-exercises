// Topic: Looping using the loop statement
// 
// Program requirements:
// * Display '1' through '4' in the terminal
// 
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable with the loop statement
// * Use break to exit the loop
fn main(){
    let mut i = 1;
    loop{
        println!("{}", i);
        if i == 4{
            break;
        }
        i += 1;
    }
    println!("✅ Done");
    while_loop();
    println!("✅  Done the While Loop");
}

fn while_loop(){
    let mut count = 1;
    while count < 4{
        println!("{}", count);
        count += 1;
    }
}
// fn main(){
//     let mut i = 1;
//     loop {
//         println!("{i}");
//         if i == 4{
//             break;
//         }
//         i += 1;
//     }
//     println!("Done!");
// }