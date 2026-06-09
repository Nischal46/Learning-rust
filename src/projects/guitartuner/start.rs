use std::io::Error;

enum GuitarString {
    E2,
    A2,
    D2,
    G2,
    B2,
    E4,
}

fn play(tune: GuitarString) -> Result<String, Error> {
    if tune.into() == "E2" {
        println!("You had pressed E2 string");
        Ok(String::from("Tunning"))
    } else {
        return Err(Error::new(std::io::ErrorKind::NotFound, "not found"));
    }
}

pub fn init() {
    println!("Guitar tuner ........");

    match play(GuitarString::E2) {
        Ok(data) => println!("Success"),
        Err(e) => println!("Error caught-----", e),
    }
}
