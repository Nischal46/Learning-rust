//NOTE: Some of the most commonly used collections are


pub fn vector_collection(){
    let vec_collection_Type = vec![1,2,3,5];
    println!("{:?}", vec_collection_Type);

    match vec_collection_Type.get(7) {
        Some(value) => println!("number index exists in {}", value),
        None => println!("number does not exist"),
    }

    fn divide(a:u8, b:u8) -> Result<u8,String> {
        if b == 0{
            Err(String::from("Value of b cannot be zero"))
        }
        else {
            Ok(a / b)
        }
    }

    match divide(4, 2) {
        Ok(value) => println!("result of divide: {}", value),
        Err(message) => println!("Err: {}", message),
    }
}
