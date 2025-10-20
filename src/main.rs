//Todo list program that allows user to add, view, delete and track the status

use tabled::{Table, Tabled};

mod input;
#[derive(Debug)]
pub enum TodoStatus {
    Completed,
    Pending,
}

impl std::fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoStatus::Completed => write!(f, "Completed"),
            TodoStatus::Pending => write!(f, "Pending"),
        }
    }
}

#[derive(Debug, Tabled)]
pub struct TodoStruct {
    pub id: u8,
    pub title: String,
    pub status: TodoStatus,
}

fn main() {
    let mut todo_list_array: Vec<TodoStruct> = Vec::new();
    loop {
        let user_input_modules = input::user_input();
        todo_list_array.push(user_input_modules);

        let table_data = Table::new(&todo_list_array);
        println!("{}", table_data)
    }
}
