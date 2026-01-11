enum UserError {
    InvalidEmailOrPassword
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    password: String
}

fn registration_user(name: String, email: String, password: String) -> User {
    return User {
        name: name,
        email: email,
        password: password
    }
}

fn login_user(email: String, password: String, user_list:&Vec<User>) -> Result<String, UserError>{
    println!("In login function");
    println!("{}", email);
    println!("{}", password);
    println!("{:?}", user_list);

    for x in user_list {
        if x.email == "baniya@gmail.com" && x.email == "abcd" {
            return Ok("Login successfully".to_string());
        }
    }
    return Err(UserError::InvalidEmailOrPassword);
}

fn main () {
    let mut user_vec: Vec<User> = Vec::new();
    let fn_call_1 = registration_user("nischal".to_string(), "nischal@gmail.com".to_string(), "admin".to_string());
    user_vec.push(fn_call_1);

    let fn_call_1 = registration_user("baniya".to_string(), "baniya@gmail.com".to_string(), "admin".to_string());
    user_vec.push(fn_call_1);
    // println!("{:#?}", obj1); //it will generate error as referencing had been transferred

    let login = login_user("baniya@gmail.com".to_string(), "abcd".to_string(), &user_vec);

    match login {
        Ok(resp) => {
            println!("{}", resp);
        }

        Err(UserError::InvalidEmailOrPassword) => {
            println!("Invalid user or password");
        }
    }

}
