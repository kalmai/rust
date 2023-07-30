use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    // let file_name = "./sample.txt";
    let file_name = "./input.txt";
    let data: Vec<String> = read_to_string(file_name).unwrap().lines().map(String::from).collect();
    let mut score_totals: i32 = 0;
    // A = rock, B = paper, C = scissors | OPPONENT CHOICE
    // rock = 1, paper = 2, scissors = 3
    //   |loss|draw|win
    //   |  X |  Y |  Z
    // A |  3 |  1 |  2
    // B |  1 |  2 |  3
    // C |  2 |  3 |  1
    let choice_map: HashMap<&str, HashMap<&str, i32>> = HashMap::from(
        [
            ("A", HashMap::from([
                ("X", 0 + 3), // win/loss value + choice needed to make that happen value
                ("Y", 3 + 1),
                ("Z", 6 + 2)
            ])),
            ("B", HashMap::from([
                ("X", 0 + 1),
                ("Y", 3 + 2),
                ("Z", 6 + 3)
            ])),
            ("C", HashMap::from([
                ("X", 0 + 2),
                ("Y", 3 + 3),
                ("Z", 6 + 1)
            ]))
        ]
    );
    for datum in data.iter() {
        if datum.len() < 2 {
            continue;
        };
        let choices: Vec<&str> = datum.split(" ").collect();
        let opponent_choice = choice_map.get(choices[0]).unwrap();
        let my_choice = opponent_choice.get(choices[1]).unwrap();
        score_totals += my_choice;
    }
    println!("{}", score_totals);
}
