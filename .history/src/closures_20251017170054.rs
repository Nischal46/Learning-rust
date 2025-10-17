pub mod main_modules {
    pub fn closure_fn_concept() {
        println!("This is closures function concept");
    }
}

pub mod database_modules {
    pub fn database_connection(url: String){
        if url == "https:dbconn" {
            print!("Database Connected Successfully");
        } else {
            println!("Failed to connect database");
        }
    }
}

pub mod hepler_modules{
    pub fn checking_voting_eligiblitiy() -> String {
        let eligible_age: u8 = 18;

    }
}
