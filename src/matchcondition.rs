pub fn match_condition() {
    //it is similar like switch condition but have more core powerful feature

    let num: u8 = 6;

    match num {
        0..6 => println!("Number is between 0 to 5"),
        6..10 => println!("Number is between 6 to 10"),
        _ => println!("Number exceeds"),
    }
}
