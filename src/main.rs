use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    todos: Vec<Todo>,
}

fn main() {
    // NOTE: Give the user some brief insturctions using println for now.
    println!("\nTodo CLI!");
    println!("1. Add new Todo");
    println!("2. Mark Todo as Complete.");
    println!("3. Remove Todo.");
    println!("4. List All Todos.");
    println!("5. Exit!");

    // NOTE: Take the user input as a string.
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input.");

    // NOTE: match the choice and give error if the choice is not matched.

    match choice.trim() {
        "1" => {
            println!("Your choice is 1. Enter new Todo");
        }
        "2" => {
            println!("Your choice is 2. Mark todo as done. ")
        }
        "3" => {
            println!("Your choice is 3. Remove Todo.");
        }
        "4" => {
            println!("Your choice is 4. List all todos");
        }
        "5" => {
            println!("Your choice is 5. Exit");
        }
        _ => {
            println!("Invalid Choice.")
        }
    }
}
