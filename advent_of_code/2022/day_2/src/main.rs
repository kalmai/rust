use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    // let file_name = "./sample.txt";
    let file_name = "./input.txt";
    let data: Vec<String> = read_to_string(file_name).unwrap().lines().map(String::from).collect();
    let mut score_totals: i32 = 0;
    // A = rock, B = paper, C = scissors | OPPONENT CHOICE
    // X = rock, Y = paper, Z = scissors | MY CHOICE
    //   |  X |  Y |  Z
    // A |  3 |  6 |  0
    // B |  0 |  3 |  6
    // C |  6 |  0 |  3
    let choice_map: HashMap<&str, HashMap<&str, i32>> = HashMap::from(
        [
            ("A", HashMap::from([
                ("X", 3 + 1),
                ("Y", 6 + 2),
                ("Z", 0 + 3)
            ])),
            ("B", HashMap::from([
                ("X", 0 + 1),
                ("Y", 3 + 2),
                ("Z", 6 + 3)
            ])),
            ("C", HashMap::from([
                ("X", 6 + 1),
                ("Y", 0 + 2),
                ("Z", 3 + 3)
            ]))
        ]
    );
    for datum in data.iter() {
        if datum.len() < 2 {
            continue;
        };
        let choices: Vec<&str> = datum.split(" ").collect();
        let my_choice = choice_map.get(choices[0]).unwrap();
        let choices_sum = my_choice.get(choices[1]).unwrap();
        score_totals += choices_sum;
    }
    println!("{}", score_totals);
}
