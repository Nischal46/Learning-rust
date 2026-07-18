use std::sync::{Arc, Mutex};

struct User {
    name: String,
    email: String
}

struct UserList {
    data: Vec<User>
}

pub fn init() {
    // concept between threaded and the real use of mutex to prevent 
    println!("Hello hello");
    
}