use std::{fs::File, io::ErrorKind};

fn main() {
    //Error Handling in Rust
    //two types of error

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Tried to create file but there was problem: {:?}", e);
                }
            }
        },
        Err(error) => {
            panic!("There was problem opening the file: {:?}", error);
        }
    };

    println!("{:?}", f);

    //shortcut for error hhandling can be done by unwrap and expect
}