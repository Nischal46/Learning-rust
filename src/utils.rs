pub fn greeting_user(name: &str){
    println!("Hello , {}", name);
}

pub struct Employee {
    pub name: String,
    pub email: String,

}

impl Employee {
    pub fn new(name: String, email: String) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string()
        }
    }

    pub fn display_details(&self){
        println!("Name: {} and Email: {}", self.name, self.email);
    }
}
