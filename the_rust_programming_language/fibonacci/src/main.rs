use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let limit: u32 = match u32::from_str(&args[1]) {
        Ok(n) => {
            match n {
                0 => { println!("number must be positive"); return }
                1 => { println!("0"); return }
                2 => { println!("0, 1"); return }
                n => { n }
            }
        }
        Err(err) => {
            println!("invalid argument defaulting, number must be positive, error: {err}");
            return
        }
    };
    let mut nums = vec![0, 1];

    for _ in 1..limit - 1 {
        let length = nums.len();
        let a = nums[length - 1];
        let b = nums[length - 2];
        nums.push(a + b)
    };

    let mut index = 0;
    while index < nums.len() {
        let num = nums[index];
        if index == nums.len() - 1 {
            println!("{}", num)
        } else {
            print!("{}, ", num)
        }
        index += 1
    }
}
