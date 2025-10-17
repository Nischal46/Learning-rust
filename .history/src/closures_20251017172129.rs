pub mod main_modules {
    pub fn closure_fn_concept() {
        println!("This is closures function concept");
    }
}

pub mod database_modules {
    pub fn database_connection(url: String){
        if url == "https:dbconn" {
            println!("Database Connected Successfully");
        } else {
            println!("Failed to connect database");
        }
    }
}

pub mod hepler_modules{
    pub fn checking_voting_eligiblitiy() {
        let eligible_age: u8 = 18;

        println!("Please enter age of user: ");
        let input_age = &mut String::from("");
        std::io::stdin().read_line(input_age).unwrap();

        let finalize_age = input_age.replace("\n", "").parse::<u8>().unwrap();

        if finalize_age >= 18 {
            println!("User eligible for voting");
        } else {
            println!("User not eligible for voting");
        }
    }
}


