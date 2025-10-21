use console::style;
use std::io;
use tabled::Tabled;

use crate::user_input::input_modules::{
    user_address_input_fn, user_contact_input_fn, user_name_input_fn,
};

#[derive(Debug, Tabled)]
pub struct UserStruct {
    pub name: String,
    pub contact: String,
    pub address: String,
}

impl UserStruct {
    pub fn add(&self) {
        println!("{}", style("\n Contact added Successfully").green());
    }
}

pub mod input_modules {
    use crate::user_input::user_input;

    pub fn user_name_input_fn() -> String {
        println!("\nEnter name: ");
        let response = user_input("username".to_string());
        response.1
    }

    pub fn user_contact_input_fn() -> String {
        println!("\nEnter contact number: ");
        let response = user_input("usercontact".to_string());
        response.1
    }

    pub fn user_address_input_fn() -> String {
        println!("\nEnter address: ");
        let response = user_input("useraddress".to_string());
        response.1
    }
}

#[allow(dead_code)]
pub fn user_input(label: String) -> (String, String) {
    let mut user_inp = String::new();
    io::stdin()
        .read_line(&mut user_inp)
        .expect("Input not provided");

    let mut response = user_inp.replace("\n", "");

    if response.is_empty() {
        if label == "username".to_string() {
            println!("Name could not be blank");
            response = user_name_input_fn();
        } else if label == "usercontact".to_string() {
            println!("Contact number could not be blank");
            response = user_contact_input_fn();
        } else {
            response = user_address_input_fn();
        }
    }

    if label == "usercontact" {
        for checknum in response.chars().into_iter() {
            if !checknum.is_numeric() {
                println!(
                    "{}",
                    style("Contact number couldnot have text. please enter correct number").red()
                );
                response = user_contact_input_fn();
                break;
            }
        }

        if response.len() != 10 {
            println!(
                "{}",
                style("\n Please enter contact number of ten digit length").red()
            );
            response = user_contact_input_fn();
        }
    }

    (label, response)
}
