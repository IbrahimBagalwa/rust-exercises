struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let response =  Survey {
        q1: Some(10),
        q2: Some(true),
        q3: Some("hello".to_string()),
    };
    match response.q1 {
        Some(q1) => println!("q1: {:?}", q1),
        None => println!("q1: No response provided"),
    }
      match response.q2 {
        Some(q2) => println!("q1: {:?}", q2),
        None => println!("q1: No response provided"),
    }
      match response.q3 {
        Some(q3) => println!("q1: {:?}", q3),
        None => println!("q1: No response provided"),
    }
}