mod helpers;

fn main() {
    let mut user_array: Vec<helpers::helpers_modules::UsersDetails> = Vec::new();
    let mut count = 1u8;
    loop {
        let result = helpers::helpers_modules::user_input();
        // println!("Displaying user input :-> Username: {} || Useremail: {}", result.0, result.1);

        let details = helpers::helpers_modules::UsersDetails {
            name: result.0,
            email: result.1,
        };
        details.add();

        user_array.push(details);

        if count == 3 {
            break;
        }

        count += 1;
    }
    println!(
        "Displaying from returning of struct from other modules, {:#?}",
        user_array
    );
}
