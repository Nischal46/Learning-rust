use std::io;

fn board_initialization(board: &Vec<[char; 3]>){
    for row in board {
        println!("{:?}", row);
    }
}

fn take_user_inp() -> i8 {
    let mut user_inp = String::new();
    io::stdin().read_line(&mut user_inp).expect("Failed to take user input");

    match user_inp.trim().parse::<i8>() {
        Ok(num) => {
            if num >= 1 && num <=9 {
                println!("Valid integer. You had type {}", num);
                num
            }
            else {
                println!("Please type a number in between 1 to 9");
                take_user_inp()
            }
        }
        Err(_) => {
            println!("Invalid integer. the given input is not integer\nPlease type integer");
            take_user_inp()
        }
    }
}

fn assign_value_in_board(board: &mut Vec<[char; 3]>, res: i8, player: &mut char) {
    match res {
        1 => {
            board[0][0] = *player;
        }
        2 => {
            board[0][1] = *player;
        }
        3 => {
            board[0][2] = *player;
        }
        _ => {
            println!("Out of the input scope")
        }
    }
}

pub fn init() {
    println!("Initializing of tic tac toe ....."); 
    let mut board = vec![['_'; 3]; 3];
    board_initialization(&board);
    let mut player = 'X';

    loop {
        let response = take_user_inp();
        assign_value_in_board(&mut board, response, &mut player);
    }
    
}