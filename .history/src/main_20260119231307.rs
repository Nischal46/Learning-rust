const STATIC_ID: &str = "static_identifier_12345";

fn main(){
    //string literal is immutable.example

    let string_literal_1: &str = "This is a string literal. It cannot be modified.";
    println!("{}", string_literal_1);
    println!("Static ID: {}", string_literal_1);

    //Rust treats string literals as &'static str and 
    //trate fn println(arg: &str) as taking a reference to a string slice.
    //So we can pass string literals directly to println! macro.

    let mut name = String::from("Nischal"); //String type is mutable and heap allocated. To mutate we must declare mut keyword.

    name.push_str(" Baniya"); //push_str method appends a string slice to the String.

    name.push(char::from('!')); //push method appends a single character to the String.

    println!("Hello, {}", name);
    println!("Hello again, {}", name);


    let name_redeclare = name; //here name is moved to name_redeclare
    //println!("Hello, {}!", name); //this will give error as name is moved
    println!("Hello once more, {}!", name_redeclare);

    let secret_product = String::from("Secret Product 001");
    move_ownership(secret_product); //here ownership of secret_product is moved to the function

    println!("Logging secret id: {}", secret_product)
    //these all will be drop automatically as the end of the scope
}

fn move_ownership(secret_item: String){
    println!("The secret item is: {}", secret_item);
}