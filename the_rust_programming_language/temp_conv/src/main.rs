use std::io;
use std::str::FromStr;

fn main() {
    let mut input = String::new();

    println!("follow format to convert from one temperature scale to the other:");
    println!("<original-scale><value><desired-scale>");
    println!("F68C");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => println!("error: {error}"),
    };
    let input = input.trim().to_string().to_lowercase();
    let values = input.split_at(input.len() - 1);
    let num = u32::from_str(values.0).unwrap();
    let scale = values.1;
    println!("{}", num);
    println!("{}", scale)
}
