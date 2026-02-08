fn main() -> Result<(), Box<dyn std::error::Error>>{
    let data_1 = "Hey developer";

    let data_2 = String::from("Rust rules");

 
    passing_ownership_of_string_literals(data_1);

    passing_ownership_of_string_of_heap_concept(data_2);

    println!("Checking if we can access {}", data_1);

    //println!("Checking for especially string that stores as heap: {}", data_2);

    Ok(())
}

fn passing_ownership_of_string_literals(data: &str){
    println!("Logging in function scope -- {}", data);
}

fn passing_ownership_of_string_of_heap_concept(data: String){
    println!("Passing of the ownership in thhhis function scope: {}", data);
    println!("Check whether: {}", data);
}
