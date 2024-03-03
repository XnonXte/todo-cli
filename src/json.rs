use crate::constants::FILE_PATH;
use models::Todo;
use std::{fs::File, io::Write};

pub mod models;

pub fn open_json_file() -> Vec<Todo> {
    let file = File::open(FILE_PATH).expect("Failed to open file!");
    let json: Vec<Todo> = serde_json::from_reader(file).expect("Failed to parse JSON!");
    json
}

pub fn write_json_file(todos: &Vec<Todo>) {
    let mut file = File::create(FILE_PATH).expect("Failed to create file!");
    let data = serde_json::to_string_pretty(&todos).expect("Failed to serialize JSON to string!");
    file.write_all(data.as_bytes())
        .expect("Failed to write data to JSON file!");
}
