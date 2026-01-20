fn main() {
    let mut initial_value = String::from("Hello, World!");
    println!("Before passing reference {}", initial_value);
    pass_ownership(&mut initial_value);

    println!("After passing reference {}", initial_value);

    let result = dangle_concept_first();
    println!("Dangle concept result: {}", result);

    let sample_string = String::from("Hello, Rustaceans!");
    find_first_word_by_byte_method(&sample_string);
    let char_concept = find_first_word_by_char_method(&sample_string);
    println!("First word by char method: {}", char_concept);
}

//concept about passing references and mutability in Rust
//mutable reference concept that allows modification of the original data
fn pass_ownership(s: &mut String) {
    println!("{}", s);
    s.push_str(" Modified inside function.");
}

//dangling references concept in Rust
//Rust prevents dangling references at compile time
//EXAMPLE

// fn dangle_concept_first() -> &String
//probaly this will give compile time error
//because new_variable will be dropped when the function ends
//and returning reference to it will create dangling reference

fn dangle_concept_first() -> String {
    let new_variable = String::from("This varible lives in this inside function scope");
    new_variable
}

fn find_first_word_by_byte_method(s: &str) -> &str {
    let bytes = s.as_bytes(); //convert string to array of bytes

    //for(i as index and &item as u8 number as 101, 104 etc) 
    for (i, &item) in bytes.iter().enumerate() {
        let ch = item as char; //converting u8 number to char
        println!("Index: {}, Item: {}, char: {}", i, item, ch);

        if item == b' ' {//checking for space character
            return &s[..i]; //this slices the string from start to index i
        }
    }
    s
}

fn find_first_word_by_char_method(s: &str) -> String {
    s.chars().take_while(|&c| c != ' ').collect()
}