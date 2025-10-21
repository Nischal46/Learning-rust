use std::io;

pub struct UserStruct {
    pub name: String,
    pub contact: String,
    pub address: String
}

pub mod input_modules {
    use crate::user_input::user_input;

    pub fn user_name_input_fn() {
        user_input("username".to_string());
    }

    pub fn user_contact_input_fn() {
        user_input("usercontact".to_string());
    }

    pub fn user_address_input_fn() {
        user_input("useraddress".to_string());
    }
}

#[allow(dead_code)]
pub fn user_input(label: String) -> (String, String) {
    let mut user_inp = String::new();
    io::stdin()
        .read_line(&mut user_inp)
        .expect("Input not provided");

    let response = user_inp.replace("\n", "");
    (label, response)
}
