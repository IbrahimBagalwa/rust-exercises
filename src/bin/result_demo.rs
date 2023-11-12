#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err(String::from("Invalid input"))
    }
}

fn print_choice(input: &MenuChoice){
    println!("Choice = {:?}", input);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}
fn main() {
    // let choice: Result<MenuChoice,_> = get_choice("mainmenu");
    // println!("choice = {:?}", choice);
    // match choice {
    //     Ok(inner_choice) => {
    //         print_choice(&inner_choice);
    //     }
    //     Err(error) => {
    //         println!("Error: {:?}", error);
    //     }
    // }
   let choice = pick_choice("ok");
   println!("choice = {:?}", choice);
}