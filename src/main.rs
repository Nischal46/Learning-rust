fn main(){

    //moving ownership to next function directly

    let s1 = 12;
    accept_ownership(s1);

    let s2 = 33;
    accept_only_reference(&s2);
    println!("The only way that we are able to get the value of s2 is because of referencing: {}", s2);

    let person_greet = greet("Nischal".to_string(), "nischal@gmail.com".to_string());
    println!("Displaying returning value from other fn, --- {}", person_greet);
}

fn accept_ownership(num: i32){
    println!("ownership moved from main to this sub function: {}", num);
}

fn accept_only_reference(num: &i32){
    println!("taking only reference: {}", num);
}

fn greet(name: String, email:String) -> String {
    let greet = format!("Welcome, Mr {} and your email is {}", name, email);
    greet
}
