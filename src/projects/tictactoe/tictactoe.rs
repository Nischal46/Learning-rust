use std::io::{self, Write};

pub fn tictactoe() {
    println!("This is tic tac toe game concept");
    init();
}

fn clear_screen() {
    // Clear screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn board(ref_pass_array: &mut [[&str; 3]; 3]) {
    println!("This is board..");
    println!("-----------------");

    for n in ref_pass_array.iter() {
        println!("{:?}", n);
    }
    //println!("Printing 3d array board -- {:?}", board_3d_array);
}

fn check_available_or_not(ref_pass_array: &mut [[&str; 3]; 3], player: &str) {
    if ref_pass_array[0][0] != "" {
        println!("This position in board is already filled");
    } else {
        println!("Empty position. filled now");
    }
}

fn take_user_input() -> i32 {
    println!("Please choose option");
    let mut choice = String::new();
    let output;
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read stdin");

    match choice.trim().parse::<i32>() {
        Ok(i) => {
            output = i;
            return output;
        }
        Err(_) => {
            println!("Please enter a valid number. Try agian!");
            take_user_input()
        }
    }
}

enum Player {
    X,
    O,
}

fn init() {
    println!("Game logic start from here --- ");
    let mut board_3d_array: [[&str; 3]; 3] = [["", "", ""], ["", "", ""], ["", "", ""]];
    let mut count = 1;
    let mut player = Player::X;

    while count < 10 {
        inserting_char_in_board(&mut board_3d_array, count);
        take_user_input();
        //clear_screen();
        board(&mut board_3d_array);
        count += 1;
    }
}

fn inserting_char_in_board(mut arr_3d: &mut [[&str; 3]; 3], inp: i32) {
    match inp {
        1 => {
            arr_3d[0][0] = "X";
            check_available_or_not(&mut arr_3d, "X");
        }
        2 => arr_3d[0][1] = "0",
        3 => arr_3d[0][2] = "X",
        4 => arr_3d[1][0] = "0",
        5 => arr_3d[1][1] = "X",
        6 => arr_3d[1][2] = "0",
        7 => arr_3d[2][0] = "X",
        8 => arr_3d[2][1] = "0",
        9 => arr_3d[2][2] = "X",
        _ => println!("Default option choose"),
    }
}
