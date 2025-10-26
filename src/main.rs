use std::collections::HashMap;

fn main() {
    //using of hashmap and its method in program
    //these type of collections are widely used in game development, scalable server

    //this will use hash function and store inside of ram memory

    let mut product_reviews = HashMap::new();

    product_reviews.insert("Laptop".to_string(), "DELL".to_string());
    product_reviews.insert("Mouse".to_string(), "Fantech".to_string());
    product_reviews.insert("Monitor".to_string(), "Benq".to_string());

    //NOTE: contains_key() method

    if product_reviews.contains_key("Monitor") {
        println!("Monitor found");
    }

    else{
        println!("Monitor doesnot found")
    }

    //NOTE: can also iter through loop

    let to_find = ["Laptop", "Keyword"];

    for &product in &to_find {
        match product_reviews.get(product) {
            Some(review) => println!("{product} : {review}"),
            None => println!("{product} not reviewed")
        }
    }

    for key in product_reviews.keys() {
        println!("{key}");
    }

    //we can also initialized through array

    let bike_arr = [("duke", 700000), ("yamaha", 600000), ("pulsar", 450000)];

    let initialize_bike = HashMap::from(bike_arr);

    // println!("Logging of bike array, {:#?}", initialize_bike);

    for (key, val) in initialize_bike.iter() {
        println!("{} :: {}", key, val);
    }

    // panic!("App crash")
}