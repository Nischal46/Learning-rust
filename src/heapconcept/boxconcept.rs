enum Command {
    Move { room: String, next: Box<Command> },
    Clean { mins: u32, next: Box<Command> },
    Quit,
}

fn start_robot_vaccum(inp: &Command) {
    match inp {
        Command::Move { room, next } => {
            println!("Robot is moving to {}", room);
            start_robot_vaccum(next);
        }

        Command::Clean { mins, next } => {
            println!("Cleaning for {} min", mins);
            start_robot_vaccum(next);
        }

        Command::Quit => println!("Robot job finished.. Quiting program"),
    }
}

pub fn logic_for_box() {
    let plan = Command::Move {
        room: "Nischal Room".to_owned(),
        next: Box::new(Command::Clean {
            mins: 10,
            next: Box::new(Command::Quit),
        }),
    };

    start_robot_vaccum(&plan);
}
