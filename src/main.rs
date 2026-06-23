fn closure_concept(x: &str, y: i32) {
    println!("This is closure concept fn");
    println!(
        "From anonymous fn clousre getting values x: {} and y: {}",
        x, y
    );
}

enum FootballPlayer {
    Messi,
    Ronaldo,
    Neymar,
}

fn main() {
    println!("Doing rust from beginning");

    let anonymous_fn_call = |x, y| {
        closure_concept(x, y);
    };

    let result = anonymous_fn_call("nischal", 97);

    let choosing_player = FootballPlayer::Messi;

    match choosing_player {
        FootballPlayer::Messi => println!("He is from Argentina"),
        _ => println!("Not in the list"),
    }

    let wrap_value = Some("Internal wrap value");
    println!("Logging of the wrap value: {}", wrap_value.unwrap());
}
