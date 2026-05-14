#[derive(Debug)]
enum Character {
    Attacker,
    Defenser,
}

#[derive(Debug)]
struct Player<'a> {
    player_character: &'a str,
    player_health: u8,
}

pub fn concept() {
    println!("This is going to be struct concept");

    let player1 = Player {
        player_character: "attacker",
        player_health: 100,
    };

    println!("Logging of the player1 ---- {:?}", player1);
    println!(
        "Logging one of the type of the enum ----- {:?}",
        Character::Attacker
    );
}
