mod constants;
mod json;
mod operations;
mod utils;

use colored::Colorize;
use operations::{create_todo, delete_todo, read_todos, update_todo};
use utils::input;

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
