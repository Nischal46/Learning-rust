// struct concept with trait

trait AuthToken {
    fn check_token(&self) -> bool;
}

#[derive(Debug)]
struct AuthLogin<'a> {
    email: &'a str,
    password: &'a str,
    token: &'a str,
}

#[derive(Debug)]
struct AuthRegister<'a> {
    name: &'a str,
    email: &'a str,
    password: &'a str,
}

impl<'a> AuthToken for AuthLogin<'a> {
    fn check_token(&self) -> bool {
        self.token == "asdfghjkl"
    }
}

pub fn struct_concept() {
    let user = AuthLogin {
        email: "nisal@gmail.com",
        password: "abcd",
        token: "asdfghjkl",
    };

    let check_token = user.check_token();

    println!("Logging user object --- {:?}", user);
    println!("check token ----{}", check_token);
}
