use serde::{Deserialize, Serialize};

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
    println!("Hello, world!");
}
