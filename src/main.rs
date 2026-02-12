//enum concept
#[derive(Debug)]
enum Role {
    admin,
    user
}

#[derive(Debug)]
enum UserDetails {
    email(String),
    phone_number(i32)
}

#[derive(Debug)]
enum PlayerMoveDirection {
    up {x: i8, y:i8},
    down {x: i8, y: i8},
    right {x: i8, y: i8},
    left {x: i8, y: i8}
}

fn main(){
    access_user_data(Role::admin);
    getting_user_details(UserDetails::email(String::from("nisal@rust.com")));

    let player_move = PlayerMoveDirection::up {
        x: 2, y:0
    };

    println!("Logging of the player movement: {:#?}", player_move);
}

fn access_user_data(role: Role){
    println!("Logging accepting parameter: {:#?}", role);
}

fn getting_user_details(userdetails: UserDetails){
    println!("Logging user details: {:#?}", userdetails);
}
