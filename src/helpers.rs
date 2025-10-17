pub mod helpers_modules {

    pub fn user_input() -> (String, String) {
        println!("Enter name: ");
        let user_name = &mut String::from("");
        std::io::stdin().read_line(user_name).unwrap();

        println!("Enter email: ");
        let user_email = &mut String::from("");
        std::io::stdin().read_line(user_email).unwrap();

        (
            user_name.to_string().replace("\n", ""),
            user_email.to_string().replace("\n", ""),
        )
    }

    #[derive(Debug)]
    pub struct UsersDetails {
        pub name: String,
        pub email: String,
    }

    impl UsersDetails {
        pub fn add(&self) {
            println!(
                "Recently added details: username: {} and useremail: {}",
                self.name, self.email
            );
        }
    }
}
