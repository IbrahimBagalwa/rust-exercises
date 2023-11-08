struct Score{
    score: i32,
}

fn main(){
    let my_score = vec![
            Score{score:90},
            Score{score:40},
            Score{score:50},
            Score{score:20},
            Score{score:95}
        ];
    for score in my_score {
        println!("score = {:?}", score.score)
    }
}