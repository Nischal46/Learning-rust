fn main(){
    //concept about lifetime 

    let ptr;
    let another_ptr;

    {
        let declare_String: &'static str = "Inside scope";
        ptr = declare_String; 

        let num : &'static i8 = &10;
        another_ptr = num;
        println!("Accessing inside: {}", num);
        println!("Accessing inside: {}", another_ptr);
    }

    // println!("Logging declare_String outside: {}", declare_String);

    println!("Accessing outside: {}", ptr);
    println!("Accessing outside: {}", another_ptr);

    let result = prevent_dangling_reference();
    println!("Accessing outside: {}", result);
}

fn prevent_dangling_reference() -> &'static str{
    let result;
    {
        // let new_lifetime : &'static str = "i am still active from differnt function";
        let new_lifetime : &'static str = "i am still active from differnt function dsf";
        result = new_lifetime;
        println!("Accessing inside: {}", new_lifetime);
    } 
    result
}
