fn main() {
    struct Gift {
        day: String,
        verse: String
    }

    let gifts: [Gift; 12] = [
        Gift { day: "first".to_string(), verse: "a partridge in a pear tree".to_string()},
        Gift { day: "second".to_string(), verse: "two turtle doves".to_string()},
        Gift { day: "third".to_string(), verse: "three French hens".to_string()},
        Gift { day: "fourth".to_string(), verse: "four calling birds".to_string()},
        Gift { day: "fifth".to_string(), verse: "five gold rings".to_string()},
        Gift { day: "sixth".to_string(), verse: "six geese a-laying".to_string()},
        Gift { day: "seventh".to_string(), verse: "seven swans a-swimming".to_string()},
        Gift { day: "eighth".to_string(), verse: "eight maids a-milking".to_string()},
        Gift { day: "ninth".to_string(), verse: "nine ladies dancing".to_string()},
        Gift { day: "tenth".to_string(), verse: "ten lords a-leaping".to_string()},
        Gift { day: "eleventh".to_string(), verse: "eleven pipers piping".to_string()},
        Gift { day: "twelfth".to_string(), verse: "twelve drummers drumming".to_string()},
    ];

    for i in 0..gifts.len() {
        println!("On the {} day of Christmas my true love sent to me", gifts[i].day);
        for j in (0..i + 1).rev() {
            if j == 0 {
                if i == 0 {
                    println!("{}\n", capitalize_first_char_of(&gifts[0].verse))
                } else {
                    println!("And {}\n", gifts[0].verse)
                }
            } else {
                println!("{}", capitalize_first_char_of(&gifts[j].verse))
            }
        }
    }
}

fn capitalize_first_char_of(str: &str) -> String {
    let (first_char, remainder) = str.split_at(1);
    first_char.to_uppercase() + remainder
}
