fn check_voting_eligibility(user: &User) -> bool {
    user.age > 18 
}

#[derive(Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let mut user_array: Vec<User> = Vec::new();
    let user1 = User {
        name: "Nischal".to_string(),
        age: 19,
    };

    let user2 = User {
        name: "Suresh".to_string(),
        age: 17,
    };
    user_array.push(user1);
    user_array.push(user2);

    for user in &user_array {
        if check_voting_eligibility(user) {
            println!("Dear {}. You are eligible to vote.", user.name);
        } else {
            println!("Dear {}. You are not eligible to vote.", user.name);
        }
    }
}
