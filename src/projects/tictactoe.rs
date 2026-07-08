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

fn mutate_player(player: &mut char) {
    if *player == 'X' {
        *player = 'O';
    }
    else if *player == 'O' {
        *player = 'X';
    }
}

fn check_cell_availble(board: &mut Vec<[char; 3]>, first_index: usize, second_index: usize, player: &mut char) {
    if board[first_index][second_index] != '_' {
        println!("Cell already occupied.......");
        return;
    } else {
        board[first_index][second_index] = *player;
        // mutate_player(player)
    }
}

fn assign_value_in_board(board: &mut Vec<[char; 3]>, res: i8, player: &mut char) {
    match res {
        1 => {
            check_cell_availble(&mut *board, 0, 0, player);
        }
        2 => {
            check_cell_availble(&mut *board, 0, 1, player);

        }
        3 => {
            check_cell_availble(&mut *board, 0, 2, player);

        }
        4 => {
            check_cell_availble(&mut *board, 1, 0, player);

        }
        5 => {
            check_cell_availble(&mut *board, 1, 1, player);

        }
        6 => {
            check_cell_availble(&mut *board, 1, 2, player);

        }
        7 => {
            check_cell_availble(&mut *board, 2, 0, player);

        }
        8 => {
            check_cell_availble(&mut *board, 2, 1, player);

        }
        9 => {
            check_cell_availble(&mut *board, 2, 2, player);

        }
        _ => {
            println!("Out of the input scope")
        }
    }
}

fn winning_possibility(board: &Vec<[char; 3]>, player: &char) {
    if board[0][0] == *player && board[0][1] == *player && board[0][2] == *player {
        println!("First column solved.... {} winner", player);
        return;    
    }
}

fn check_winner(board: &Vec<[char; 3]>, player: &char) {
    println!("Logging player {}", player);
    // if board[0][0] == *player && board[0][1] == *player && board[0][2] == *player {
    //     println!("First column solved.... {} winner", player);
    //     return;
    // }

    winning_possibility(&board, &player);
}

pub fn init() {
    println!("Initializing of tic tac toe ....."); 
    let mut board = vec![['_'; 3]; 3];
    let mut player = 'X';
    
    loop {
        board_initialization(&board);
        let response = take_user_inp();
        assign_value_in_board(&mut board, response, &mut player);
        check_winner(&board, &player);
        mutate_player(&mut player);

    }
    
}