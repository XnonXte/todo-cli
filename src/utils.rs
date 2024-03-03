use nanoid::nanoid;
use std::io::{stdin, stdout, Write};

pub fn gen_random_hex(length: usize) -> String {
    let hex: Vec<char> = "123456789ABCDEF".chars().collect();
    nanoid!(length, &hex)
}

pub fn input(message: colored::ColoredString) -> String {
    print!("{}", message);
    stdout().flush().expect("Failed to flush stdout!");
    let mut tmp = String::new();
    stdin().read_line(&mut tmp).expect("Failed to read line!");
    tmp.trim_end().to_string()
}
