use std::collections::{HashMap, hash_map};

pub fn concept() {
    let mut hash_map = HashMap::new();
    //there would be filed anme and filed value

    hash_map.insert("language", "rust");
    hash_map.insert("framework", "axum");
    println!("Logging of hash map -----{:?}", hash_map);

    if hash_map.contains_key("framework") {
        println!("Yes it contains in hash map");
    } else {
        println!("No it doesnot contains");
    }

    if let Some(value) = hash_map.get("framework") {
        println!("Yes it exists---{}", value);
    } else {
        println!("No it doesnot exists");
    }

    hash_map.entry("learning_curve").or_insert("hard");
    hash_map.entry("dev").or_insert("nischal");
    //hash_map.remove_entry("framework"); //remove key and value from hashmap
    hash_map.try_reserve(3);

    hash_map.entry("low_level").or_insert_with_key(|key| {
        if *key == "low_level" {
            "operating system kernel"
        } else {
            "web development"
        }
    });

    println!("Logging final hash map ---- {:#?}", hash_map);
}
