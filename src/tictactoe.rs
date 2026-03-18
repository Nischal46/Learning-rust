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

    fn user_input() {
        let mut number: i32;
        println!("Enter a choice: ");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim().parse::<i32>() {
            Ok(num) => {
                number = num;
            }
            Err(_) => {
                println!("Please enter a valid number. Try again!")
            }
        }

        // if number < 1 || number > 9 {
        //     println!("Please enter between 1 to 9");
        // }

        println!("you had enter {}.", choice);
    }

    fn init() {
        //board();
        loop {
            user_input();
        }
    }

    init();
}
