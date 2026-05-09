//NOTE: simple example how we can use box

enum Command {
    Move {
        x: f32,
        y: f32,
        z: f32,
        next: Box<Command>,
    },
    Hover {
        seconds: u32,
        next: Box<Command>,
    },
    Finish,
}

fn execute(cmd: &Command) {
    match cmd {
        Command::Move { x, y, z, next } => {
            println!("Drone is moving in {} {} {}", x, y, z);
            execute(next);
        }
        Command::Hover { seconds, next } => {
            println!("Hovering for {} seconds!!", seconds);
            execute(next);
        }

        Command::Finish => println!("Drone is about to land"),
    }
}

pub fn concept() {
    println!("This is box concet");
}
