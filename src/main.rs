use chrono::{Local, TimeZone};
use colored::Colorize;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{stdin, stdout, Write};

static FILE_PATH: &str = "./todos.json";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    id: String,
    label: String,
    content: String,
    created_timestamp: i64,
    updated_timestamp: i64,
}

impl Todo {
    fn new(label: &str, content: &str) -> (Self, String) {
        let id = gen_random_hex(8);
        let created_timestamp = Local::now().timestamp();
        let updated_timestamp = Local::now().timestamp();
        (
            Todo {
                id: id.clone(),
                label: String::from(label),
                content: String::from(content),
                created_timestamp,
                updated_timestamp,
            },
            id,
        )
    }
}

fn open_json_file() -> Vec<Todo> {
    let file = File::open(FILE_PATH).expect("Failed to open file!");
    let json: Vec<Todo> = serde_json::from_reader(file).expect("Failed to parse JSON!");
    json
}

fn write_json_file(todos: &Vec<Todo>) {
    let mut file = File::create(FILE_PATH).expect("Failed to create file!");
    let data = serde_json::to_string_pretty(&todos).expect("Failed to serialize JSON to string!");
    file.write_all(data.as_bytes())
        .expect("Failed to write data to JSON file!");
}

fn gen_random_hex(length: usize) -> String {
    let hex: Vec<char> = "123456789ABCDEF".chars().collect();
    nanoid!(length, &hex)
}

fn input(message: colored::ColoredString) -> String {
    print!("{}", message);
    stdout().flush().expect("Failed to flush stdout!");
    let mut tmp = String::new();
    stdin().read_line(&mut tmp).expect("Failed to read line!");
    tmp.trim_end().to_string()
}

fn read_todos() {
    let todos = open_json_file();

    if todos.len() == 0 {
        println!("Empty :(");
        return;
    }

    println!(
        "{}",
        "==================== Ongoing Todos ====================".bold()
    );
    for (index, todo) in todos.iter().enumerate() {
        let padding_length = format!("{}. ", index + 1).len();
        let padding = " ".repeat(padding_length);

        println!("{}. {}", index + 1, todo.id.green().bold());
        println!("{}{}", padding, todo.label.cyan().bold());
        println!("{}{}", padding, todo.content.cyan());
        println!(
            "{}{}{}",
            padding,
            "Created: ",
            Local
                .timestamp_opt(todo.created_timestamp, 0)
                .unwrap()
                .format("%H:%M %d/%m/%Y")
                .to_string()
                .yellow()
        );
        println!(
            "{}{}{}",
            padding,
            "Updated: ",
            Local
                .timestamp_opt(todo.updated_timestamp, 0)
                .unwrap()
                .format("%H:%M %d/%m/%Y")
                .to_string()
                .yellow()
        );
        println!();
    }
}

fn create_todo(label: &str, content: &str) {
    let (todo, id) = Todo::new(label, content);
    let mut todos = open_json_file();
    todos.push(todo);
    write_json_file(&todos);
    println!(
        "{} {}",
        "Successfully created new todo with ID:".green(),
        id
    );
}

fn update_todo(index: usize, label: &str, content: &str) -> bool {
    let mut todos = open_json_file();

    if index >= todos.len() {
        return false;
    }

    let todo = &mut todos[index];
    todo.label = String::from(label);
    todo.content = String::from(content);
    todo.updated_timestamp = Local::now().timestamp();
    write_json_file(&todos);
    true
}

fn delete_todo(index: usize) -> bool {
    let mut todos = open_json_file();

    if index >= todos.len() {
        return false;
    }

    todos.remove(index);
    write_json_file(&todos);
    true
}

fn main() {
    println!("{}", "Welcome to Todo CLI!".bright_white().on_blue().bold());
    println!("{}", "(C) 2024 XnonXte".bright_white().on_blue().bold());
    println!();

    loop {
        let operation = input("Enter an operation ([C]reate, [R]ead, [U]pdate, [D]elete): ".blue());

        match operation.to_lowercase().as_str() {
            "c" => {
                let label = input("Label: ".blue());
                let content = input("Content: ".blue());
                create_todo(&label, &content)
            }
            "r" => {
                read_todos();
            }
            "u" => {
                let index: usize = input("Todo index (one-based index): ".blue())
                    .parse()
                    .unwrap();
                let label = input("Label: ".blue());
                let content = input("Content: ".blue());

                match update_todo(index - 1, &label, &content) {
                    true => println!(
                        "{} {}",
                        "Successfully updated todo on index:".green(),
                        index
                    ),
                    false => println!("{}", "Index out of range!".red()),
                }
            }
            "d" => {
                let index: usize = input("Todo index (one-based index): ".blue())
                    .parse()
                    .unwrap();

                match delete_todo(index - 1) {
                    true => println!(
                        "{} {}",
                        "Successfully deleted todo on index:".green(),
                        index
                    ),
                    false => println!("{}", "Index out of range!".red()),
                }
            }
            _ => {
                println!("{}", "Not a valid operation! Please try again...".red());
                continue;
            }
        }
    }
}
