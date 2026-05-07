trait CheckEmail {
    fn validate_email(&self) -> String;
    fn check_citizen(&self);
}

#[derive(Debug)]
struct Employee {
    name: String,
    email: String,
    citizen: String,
}

impl CheckEmail for Employee {
    fn validate_email(&self) -> String {
        if self.email.contains(".") && self.email.contains("@") {
            println!("authentic email");
        } else {
            println!("Invalid email");
        }

        return "Finished fn check".to_string();
    }

    fn check_citizen(&self) {
        if self.citizen == "nepali" {
            println!("Congratulations. You are eligible");
        } else {
            println!("Sorry. You are not eligible.")
        }
    }
}

impl Employee {
    fn details(&self) {
        println!("Hello {}, warm greeting.", self.name);
    }
}

pub fn concept() {
    println!("This is the trait concept");

    let user = Employee {
        name: "Nischal".to_string(),
        email: "nischalgmail.com".to_string(),
        citizen: "nepali".to_string(),
    };

    println!("Logging of thhe object of the employee: {:?}", user);
    user.validate_email();
    user.check_citizen();
    user.details();
}
