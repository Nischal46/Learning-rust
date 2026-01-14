//'static concept

fn checking_lifetime(){
    let new_var: &'static str = "This is lifetime variable";
    passing_lifetime(new_var);
    println!("{}", new_var);

    let other_var: &str = "only string";

    let generated_text = "hello hello".to_string();
    passing_string(generated_text , other_var);
    // println!("{}", generated_text); //generate error because of passing reference and it doesnot clone
    println!("{}", other_var); //doesnot throw error because auto clone
}

fn passing_lifetime(inp: &str){
    println!("{}", inp);
    println!("Inside passing_lifetime: {}", main_access_var);
}

fn passing_string(inp: String, next_string: &str){
    println!("{}", inp);
    println!("Inside passing_string: {}", main_access_var);
    println!("&str ---- {}", next_string)

}

//must be uppercase according to standard
const main_access_var: &'static str = "This is global variable";

#[derive(Debug)]
enum Role {
    Superadmin,
    Admin,
    User
}

fn role(role_str: &str) -> Result<Role, String> {
    match role_str.to_lowercase().as_str() {
        "superadmin" => Ok(Role::Superadmin),
        "admin" => Ok(Role::Admin),
        "user" => Ok(Role::User),
        _ => Err(format!("Invalid role: {}", role_str)),
    }
}

fn main() {
    let r1 = role("admin");
    let r2 = role("manager");

    println!("{:?}", r1); // Ok(Admin)
    println!("{:?}", r2); // Err("Invalid role: manager")
}
