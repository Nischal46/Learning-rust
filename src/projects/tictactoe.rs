use std::{thread, time::Duration};

#[derive(Debug)]
enum Player {
    X,
    Y
}

fn board_display(board: &[[char; 3]]) {
    println!("Initializing Board...");
    for row in board.iter() {
        println!("{:?}", row);
    }
}

pub fn init() {
    let mut board = vec![['_'; 3]; 3];
    let mut count = 1;
    let mut player_turn = 'X';
    board[0][0] = player_turn;
    board_display(&board);
    

    println!("Logging player: {:?}", player_turn);

}