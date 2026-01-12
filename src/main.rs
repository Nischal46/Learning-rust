//concept for vehicle registration

enum ValidationError {
    InvalidCitizenship,
    InvalidMobileNumber,
    InvalidPANNumber,
    InvalidNIDNumber,
    InvaledLicenseNumber
}

#[derive(Debug)]
struct citizenship {
    name: String,
    citizen_number: i64
}

fn check_citizenship(){
    let registered_citizen: Vec<citizenship> = vec![
        citizenship{
            name: "Nischal Baniya".to_string(),
            citizen_number: 13432
        },
        citizenship{
            name: "Nischal Baniya".to_string(),
            citizen_number: 13432
        }
    ];

    println!("Logging saved data: -> {:#?}", registered_citizen);
}

fn check_mobilenumber(){}

fn check_pan(){}

fn check_nid(){}

fn main() {
    //this program helps user to register in the Hami nepali app where real logic mimics like
    // sending of otp, some fake data would stored in vec data type.
    // suppose all of them are different department and try to communicate with them
    check_citizenship();
}
