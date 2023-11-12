//  Topic: Option
// 
// Requirements:
// * Print out the details of the student's locker assignment
// * Lockers use numbers and are optional for students
// 
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct StudentInfo {
    name: String,
    locker: Option<i32>,
}

fn main () {
    let info = StudentInfo {
        name: String::from("John"),
        locker: Some(10),
    };
    match info.locker {
        Some(locker) => println!("Locker: {:?}", locker),
        None => println!("Locker: No response provided"),
    }
      match info.name {
       name => println!("Name: {:?}", name)
    }
}