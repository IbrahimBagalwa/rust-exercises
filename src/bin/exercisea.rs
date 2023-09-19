fn parse(code: &str) -> Vec<i32>{
    let mut initial_value = 0;
    let mut result = Vec::new();
   for st in code.chars(){
       match st{
        'i' => initial_value += 1,
        'd' => initial_value -= 1,
        's' => initial_value *= initial_value,
        'o' => result.push(initial_value),
        _ => todo!("Invalid")

       };
    }
    result
}
fn main(){
    let code = "iiisdoso";
    let output = parse(code);
    println!("{:?}", output);
}