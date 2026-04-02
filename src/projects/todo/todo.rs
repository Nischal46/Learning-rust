use std::io;

#[derive(Debug)]
struct TodoList {
    id: u16,
    task: Box<str>,
    completed: bool,
}

pub fn main_todo() {
    let mut todo_list: Vec<TodoList> = vec![];

    fn init(todo_list: &mut Vec<TodoList>) {
        loop {
            todo_menu();

            let user_choose_input = user_input_menu();

            match user_choose_input {
                1 => {
                    let res = user_task_input();
                    let count = todo_list.len();

                    let task_details = TodoList {
                        id: count as u16 + 1,
                        task: res.into_boxed_str(),
                        completed: false,
                    };

                    todo_list.push(task_details);
                }

                2 => {
                    println!("Reading the list of task ----");
                    println!("{:?}", todo_list);
                }

                5 => {
                    break;
                }

                _ => println!("J paye tei"),
            }
        }
    }

    init(&mut todo_list);
}

fn todo_menu() {
    println!("--------------Todo App--------------");
    println!("-> Press 1 to Add task");
    println!("-> Press 2 to view task");
    println!("-> Press 3 as mark done for completed task");
    println!("-> Press 4 to store in file");
    println!("-> Press 5 to quit app");
}

fn user_input_menu() -> i16 {
    let mut choice = String::new();
    loop {
        println!("Please enter a choice: ");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read user input");

        match choice.trim().parse::<i16>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. please input number"),
        }
    }
}

fn user_task_input() -> String {
    let mut task = String::new();
    io::stdin()
        .read_line(&mut task)
        .expect("failed to read user input");

    println!("You have type task as: {}", task);

    task
}
