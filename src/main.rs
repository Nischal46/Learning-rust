//enum with match Control flow operator

#[allow(dead_code)]
enum BikeTypes {
    Bajaj,
    Ktm,
    Yamaha,
    Hero,
}

fn print_bike_info() {
    println!(
        "
    Press 1. Bajaj
    Press 2. KTM
    Press 3. Hero,
    Press 4. Hero
    "
    );
}

fn main() {
    println!("Select Bike: ");
    print_bike_info();

    println!("Please select above one: ");

    let mut user_select = &mut String::from("");
    std::io::stdin().read_line(user_select).unwrap();

    let user_choice = user_select.replace("\n", "").parse::<i32>().unwrap();

    println!("user selects: {}", user_select);

    //Incase if we donot have value previous we should not have mut keyword
    let result;
    match user_choice {
        1 => result = user_choice_bike(BikeTypes::Bajaj),
        2 => result = user_choice_bike(BikeTypes::Ktm),
        3 => result = user_choice_bike(BikeTypes::Hero),
        4 => result = user_choice_bike(BikeTypes::Yamaha),
        _ => result = "Invalid typed".to_string(),
    }

    println!("User selected bike: {}", result);

    // let user_choice_bike = user_choice_bike(BikeTypes::Bajaj);
    // println!("Selected Bike: {}", user_choice_bike);
}

fn user_choice_bike(bike: BikeTypes) -> String {
    match bike {
        BikeTypes::Bajaj => "pulsar220".to_string(),
        BikeTypes::Ktm => "duke 200".to_string(),
        BikeTypes::Hero => "x-pulse 200".to_string(),
        BikeTypes::Yamaha => "fz-150".to_string(),
    }
}
