const STATIC_ID: &str = "static_identifier_12345";

fn main(){
    //string literal is immutable.example

    let string_literal_1: &str = "This is a string literal. It cannot be modified.";
    println!("{}", string_literal_1);
    println!("Static ID: {}", string_literal_1);

    //Rust treats string literals as &'static str and 
    //trate fn println(arg: &str) as taking a reference to a string slice.
    //So we can pass string literals directly to println! macro.

    let name = String::from("Nischal");

    println!("Hello, {}!", name);
    println!("Hello again, {}!", name);


    let name_redeclare = name; //here name is moved to name_redeclare
    println!("Hello once more, {}!", name);

    //these all will be drop automatically as the end of the scope
}