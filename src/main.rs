mod helpers;
mod closures;

#[derive(Debug)]
enum IpAddressKind {
    v4,
    v6
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String
}

//unit struct
#[derive(Debug)]
struct Developer;

fn main() {
    // let mut user_array: Vec<helpers::helpers_modules::UsersDetails> = Vec::new();
    // let mut count = 1u8;
    // loop {
    //     let result = helpers::helpers_modules::user_input();
    //     // println!("Displaying user input :-> Username: {} || Useremail: {}", result.0, result.1);

    //     let details = helpers::helpers_modules::UsersDetails {
    //         name: result.0,
    //         email: result.1,
    //     };
    //     details.add();

    //     user_array.push(details);

    //     if count == 3 {
    //         break;
    //     }

    //     count += 1;
    // }
    // println!(
    //     "Displaying from returning of struct from other modules, {:#?}",
    //     user_array
    // );

    closures::closures_modules::closure_concept();

    let IpAddress1 = IpAddress {
        kind: IpAddressKind::v4,
        address: String::from("192.168.0.1")
    };

    println!("Logging of ip address, {:#?}", IpAddress1);

    //unit struct just used as signal
    let dev1 = Developer;
    println!("{:#?}", dev1)
}
 