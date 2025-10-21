use std::io;
mod user_input;

fn display_details() {
    println!(
        "
    Press 1. Add Contact 
    Press 2. See List
    Press 3. To quit
    "
    )
}

fn user_menu_choice() -> u8 {
    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Error with io modules");

    let response = user_choice.replace("\n", "").parse::<u8>().unwrap();
    response
}

fn main() {
    let contact_array: Vec<user_input::UserStruct> = Vec::new();
    println!("Welcome to Call Manager App");

    loop {
        display_details();
        let user_choice = user_menu_choice();

        match user_choice {
            1 => {}

            2 => {}

            _ => {}
        }
    }
}
