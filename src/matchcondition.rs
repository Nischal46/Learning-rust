pub fn match_condition() {
    //it is similar like switch condition but have more core powerful feature

    let num: u8 = 6;

    match num {
        0..6 => println!("Number is between 0 to 5"),
        6..10 => println!("Number is between 6 to 10"),
        _ => println!("Number exceeds"),
    }

    let month = "March";

    match month {
        "January" | "February" => println!("Trigger first month"),
        "March" | "April" => println!("Trigger second month"),
        _ => println!("Default"),
    }
}
