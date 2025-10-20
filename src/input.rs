use crate::{TodoStatus, TodoStruct};
use rand::{Rng, rng};

pub fn user_input() -> TodoStruct {
    println!("Enter todo title");
    let todo_title = &mut String::from("");
    std::io::stdin().read_line(todo_title).unwrap();

    let obj = TodoStruct {
        id: rng().random_range(1..20),
        title: todo_title.to_string().replace("\n", ""),
        status: TodoStatus::Pending,
    };

    obj
}
