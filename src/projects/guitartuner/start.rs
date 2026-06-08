use std::io::{Error, ErrorKind};
use std::thread;
use std::time::Duration;

fn load_config(inp: u8) -> Result<String, std::io::Error> {
    if inp == 2 {
        println!("Great! Exact number found.");
    } else {
        // We can manually create an error if the input isn't what we want
        return Err(Error::new(ErrorKind::InvalidInput, "Number was not 2"));
    }

    // Example: Attempting to read a file that might not exist
    // The '?' operator will return the error automatically if this fails
    println!("Sleeping main thread for 3 sec.........");
    thread::sleep(Duration::from_secs(3));
    let response = std::fs::read_to_string("afile2.txt")?;

    print!("log just below error stack after 3 sec...........");

    // If we reach here, everything succeeded!
    Ok(response)
}

pub fn init() {
    // Handling the Result from load_config using a match statement
    match load_config(2) {
        Ok(data) => println!("Config loaded successfully: {}", data),
        Err(e) => eprintln!("Failed to load config: {}", e),
    }

    println!("This is my project");
    match login("admin") {
        Ok(data) => println!("{}", data),
        Err(e) => println!("Error occured: {}", e),
    }
}

fn login<'a>(role: &'a str) -> Result<&'a str, Error> {
    if role == "admin" {
        Ok("Login successfully. accessing admin portal")
    } else {
        return Err(Error::new(
            ErrorKind::PermissionDenied,
            "role not permitted granted",
        ));
    }
}
