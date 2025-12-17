use std;
pub fn fn_call_from_other_module() {
    println!("fn_call_from_other_module");
}

pub fn check_for_ownership() -> u8 {
    let num = 23;
    num
}

#[derive(Debug)]
pub struct EmployeeDetails {
    name: String,
    email: String,
}

pub fn gather_details() -> EmployeeDetails {
    let mut name = String::new();
    println!("Enter name: ");
    std::io::stdin().read_line(&mut name);

    let mut email = String::from(name.trim());
    println!("Enter email: ");
    std::io::stdin().read_line(&mut email);

    let output = EmployeeDetails{
        name: name.to_string(),
        email: email.to_string(),
    };

    output
}