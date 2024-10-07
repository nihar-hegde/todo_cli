use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

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

impl TodoList {
    fn new() -> Self {
        TodoList { todos: Vec::new() }
    }

    fn add_todo(&mut self, title: String) {
        let id = self.todos.len() + 1;
        let todo = Todo {
            id,
            title,
            completed: false,
        };

        self.todos.push(todo);
    }

    fn complete_todo(&mut self, id: usize) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            true
        } else {
            false
        }
    }
}

// NOTE: function to save todos from the vector to a json file.
// this function will return a emtpy result if it is Ok or an error.
fn save_todos(todo_list: &TodoList) -> io::Result<()> {
    let json = serde_json::to_string(&todo_list).unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("todos.json")?;

    file.write_all(json.as_bytes())?;

    Ok(())
}

fn load_todos() -> io::Result<TodoList> {
    let path = Path::new("todos.json");
    if path.exists() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let todo_list: TodoList = serde_json::from_str(&contents)?;
        Ok(todo_list)
    } else {
        Ok(TodoList::new())
    }
}

fn main() -> io::Result<()> {
    let mut todo_list = load_todos()?;

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
            println!("1.Enter new Todo Title!");
            let mut title = String::new();
            io::stdin()
                .read_line(&mut title)
                .expect("Failed to read input.");
            todo_list.add_todo(title.trim().to_string());
            println!("Todo Added.");
        }
        "2" => {
            println!("2. Enter Id of the todo to be marked as complete. ");
            let mut id = String::new();
            io::stdin()
                .read_line(&mut id)
                .expect("Failed to read input.");
            if let Ok(id) = id.trim().parse() {
                if todo_list.complete_todo(id) {
                    println!("Todo Completed.");
                } else {
                    println!("Todo not found!");
                }
            }
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
    save_todos(&todo_list)?;

    Ok(())
}
