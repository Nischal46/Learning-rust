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

#[derive(Debug)]
struct Product {
    title: &'static str,
    price: i32
}

impl Product {
    fn new(title: &'static str, price: i32) -> Self {
        Self { title, price }
    }
}

fn check_mobilenumber(){}

fn check_pan(){}

fn check_nid(){}

fn main() {
    //this program helps user to register in the Hami nepali app where real logic mimics like
    // sending of otp, some fake data would stored in vec data type.
    // suppose all of them are different department and try to communicate with them
    check_citizenship();

    let mut string1 = "nischal".to_string();

    let string2 = "b";

    string1.push('w');

    println!("{}", string1);
    println!("Can be accessed after merging {}", string2);

    let s3 = string1;

    println!("new string: {}", s3);

    let newObj = Product::new("Dell Black Laptop", 12000);

    println!("Logging of prduct obj: {:#?}", newObj);

    //cannot print because its reference had been moved to new variable
    // println!("trying to log string1 {}", string1)
}
