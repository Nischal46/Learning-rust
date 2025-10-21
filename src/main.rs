use crate::user_input::UserStruct;
use colored::Colorize;
use console::style;
use std::io;
use tabled::Table;
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

fn user_menu_choice() -> String {
    let mut user_choice = String::new();
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Error with io modules");

    let response = user_choice.replace("\n", "");
    response
}

fn app_run_flag(inp: &mut bool) {
    *inp = false;
}

fn check_type(input: &String) -> &'static str {
    if input.parse::<i32>().is_ok() {
        "integer"
    } else if input.parse::<f64>().is_ok() {
        "float"
    } else {
        "string"
    }
}

fn main() {
    let mut contact_array: Vec<user_input::UserStruct> = Vec::new();
    println!(
        "{}",
        "\n -- ********** Welcome to Call Manager App ********** --".blue()
    );
    let mut app_run = true;

    while app_run {
        display_details();
        let user_choice = user_menu_choice();

        let check_type = check_type(&user_choice);

        if check_type == "integer" {
            match user_choice.parse::<i8>() {
                Ok(1) => {
                    let user_name = user_input::input_modules::user_name_input_fn();
                    let user_contact = user_input::input_modules::user_contact_input_fn();
                    let user_address = user_input::input_modules::user_address_input_fn();

                    let contact_details = UserStruct {
                        name: user_name,
                        contact: user_contact,
                        address: user_address,
                    };

                    contact_details.add();
                    contact_array.push(contact_details);
                }

                Ok(2) => {
                    let result = Table::new(&contact_array);
                    println!("{}", result);
                    if contact_array.len() == 0 {
                        println!("{}", style("Empty data").yellow());
                    }
                }

                Ok(3) => {
                    app_run_flag(&mut app_run);
                    println!("{}", style("\n Quit App Successfully").green());
                }

                _ => {
                    println!(
                        "{}",
                        style("\n Invalid input. Please choose appropriate").yellow()
                    );
                }
            }
        } else {
            println!("{}", style("\n Please use number as above description").red());
        }
    }
}
