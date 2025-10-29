use std::panic;

fn check_condition() -> Result<(), String> {
    if 2 < 3 {
        Err("Sorry wrong".to_string())
    } else {
        Ok(())
    }
}

fn main() {
    //NOTE: Rust has two type of error - recoverable and unrecoverable

    match check_condition() {
        Ok(_) => println!("All good"),
        Err(e) => println!("Error caught {}", e),
    }

    //since panic crash and quit whole program thread so it has catch_unwind method

    let result = panic::catch_unwind(|| {
        if 2 < 3 {
            panic!("Sorry darling");
        } else {
            println!("True true");
        }
    });

    match result {
        Ok(_) => println!("No panic occurred"),
        Err(err) => println!("Recovered from panic: {:?}", err),
    }

    let v = vec![2,3,4,5];

    let check = panic::catch_unwind(|| {
        let num = v[3];
    });

    match check {
        Ok(_) => println!("No panic occurred"),
        Err(err) => println!("Recovered from panic: {:?}", err),
    }

    println!("Program continues!");
}
