use std::{io, thread, time::Duration};

fn board_display(board: &[[char; 3]]) {
    println!("Initializing Board...");
    for row in board.iter() {
        println!("{:?}", row);
    }
}

fn option(player: &mut char) {
    match player {
        'X' => {
            println!("Passes {}", player);
            *player = 'Y';
        }
        'Y' => {
            println!("Passes {}", player);
            *player = 'X';
        },
        _ => println!("Jpt")
    }
}

fn take_input_from_user() -> Result<u8, Box<dyn std::error::Error>>{
    println!("Please give input: ");
    let mut input = String::new();
    io::stdin().read_line(&mut  input).expect("Failed to get input data");
    let num_inp = input.trim().parse::<u8>()?;

    println!("User had type: {}", num_inp);
    num_inp
}

pub fn init() {
    let mut board = vec![['_'; 3]; 3];
    let mut count = 1;
    let mut player_turn = 'X';
    board[0][0] = player_turn;
    board_display(&board);

    loop {
        option(&mut player_turn);
        take_input_from_user();
    }
   
}