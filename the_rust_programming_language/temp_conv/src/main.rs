use std::io;
use std::str::FromStr;

fn main() {
    let mut input = String::new();

    println!("follow format to convert from one temperature scale to the other:");
    println!("<value><desired-scale-case-insensitive>");
    println!("68C converts 68 fahrenheit to 20 celsius for example");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(error) => println!("error: {error}"),
    };
    let input = input.trim().to_string().to_lowercase();
    let values = input.split_at(input.len() - 1);
    let num = f32::from_str(values.0).unwrap();
    let scale = values.1;

    match scale {
        "c" => {
            println!("{input} is {:.1}F", ((num - 32.0) / 1.8))
        }
        "f" => {
            println!("{input} is {:.1}C", num * 1.8 + 32.0)
        }
        &_ => {
            println!("scale not supported")
        }
    };
}
