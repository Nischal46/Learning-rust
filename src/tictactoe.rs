use std::io;

pub fn game() {
    //initialize board

    fn board() {
        let mut board: [[char; 3]; 3] = [[' '; 3]; 3];
        println!("-------------");
        for row in 0..3 {
            print!("|");

            for column in 0..3 {
                print!(" {} |", board[row][column]);
            }
            println!("\n-------------")
        }
    }

    fn user_input() -> i32 {
        let mut number: i32;
        println!("Enter a choice: ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().parse::<i32>() {
            Ok(num) => {
                if num < 1 || num > 9 {
                    println!("Please type number from 1 to 9");
                    user_input()
                } else {
                    number = num;
                    return number;
                }
            }
            Err(_) => {
                println!("Please enter a valid number. Try again!");
                user_input()
            }
        }
    }

    fn logic() {}

    fn init() {
        //board();
        let mut res;
        loop {
            res = user_input();
            println!("User hhad type: {}", res);
        }
    }

    init();
}
