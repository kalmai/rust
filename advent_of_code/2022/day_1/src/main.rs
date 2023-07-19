use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;

fn main() {
    let mut food_items: Vec<String> = Vec::new();
    // if let Ok(lines) = read_lines("./sample.txt") {
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok (num) = line {
                food_items.push(num);
            }
        }
    }

    let mut n = 0;
    let mut all_calorie_totals: Vec<i32> = Vec::new();
    let mut single_calorie_total: Vec<i32> = Vec::new();

    while n < food_items.len() {
        if food_items[n] != "" {
            single_calorie_total.push(food_items[n].parse::<i32>().unwrap());
        } else {
            let mut calorie_sum = 0;
            for calorie in single_calorie_total.iter() {
                calorie_sum += calorie;
            }
            all_calorie_totals.push(calorie_sum);
            single_calorie_total.clear();
        }
        n += 1;
    }
    let mut top_three_calories = vec![0];
    for &calories in all_calorie_totals.iter() {
        match calories.cmp(&top_three_calories[0]) {
            Ordering::Less => {},
            Ordering::Greater => {
                top_three_calories.push(calories);
                if top_three_calories.len() > 3 {
                    top_three_calories.sort();
                    top_three_calories.remove(0);
                }
            },
            Ordering::Equal => {}
        }
    }
    let sum: i32 = top_three_calories.iter().sum();
    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
