mod utils;

fn main(){
    //NOTE:
    let fn_call = passing_reference();
    println!("accepting ownership pass by fn by returning---- {}", fn_call);

    //NOTE: mutating concept of referncing

    let mut s1 = String::from("First string declared");

    {
        let s2 = &mut s1;
        *s2 = String::from("Changed string");
    }

    let s3 = &s1;

    println!("s3: {}", s3);

    utils::greeting_user("Nischal");

    let employee1 = utils::Employee::new("Nischal".to_string(), "nischal@gmail.com".to_string());
    employee1.display_details();

}

fn passing_reference() -> String {
    let string_in_scope = String::from("String declared in passing reference function");
    string_in_scope
}
