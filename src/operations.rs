use crate::json::models::Todo;
use crate::json::{open_json_file, write_json_file};
use chrono::{Local, TimeZone};
use colored::Colorize;

pub fn read_todos() {
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

pub fn create_todo(label: &str, content: &str) {
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

pub fn update_todo(index: usize, label: &str, content: &str) -> bool {
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

pub fn delete_todo(index: usize) -> bool {
    let mut todos = open_json_file();

    if index >= todos.len() {
        return false;
    }

    todos.remove(index);
    write_json_file(&todos);
    true
}
