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

fn init() {
    println!("Game logic start from here --- ");
    let mut board_3d_array: [[&str; 3]; 3] = [["", "", ""], ["", "", ""], ["", "", ""]];
    let mut count = 1;

    while count < 10 {
        clear_screen();
        inserting_char_in_board(&mut board_3d_array, count);
        board(&mut board_3d_array);
        count += 1;
    }
}

fn inserting_char_in_board(arr_3d: &mut [[&str; 3]; 3], inp: i32) {
    match inp {
        1 => {
            arr_3d[0][0] = "X";
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
