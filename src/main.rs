//use std::process::Command;
use std::fs;
use std::path::Path;
use std::io;
use std::io::prelude::*;

mod utils;
mod vector;
mod errorhandling;

fn main() -> io::Result<()> {
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

    if Path::new("/dev/video0").exists() {
        println!("Camera detected");
    } else {
        println!("Camera not detected");
    }

    //NOTE: Reading of the hardware of the system

    //for entry in fs::read_dir("/dev")? {
    //    let entry = entry?;
    //    let file_name = entry.file_name();
    //    let name = file_name.to_string_lossy();
    //
    //    println!("Logging {:?}", entry);
    //}

    let f = fs::File::open("mytext.txt")?;

    println!(" Reading of the file: {:?}", f);
    
    //let reader = io::BufReader::new(f);
    //
    //for line in reader.lines() {
    //    println!("{}", line?);
    //}
    
    let contents = include_str!("../mytext.txt");

    println!("Reading of the content of the my text file {:?}", contents);

    let num_array = [3, 4, 6, 7];

    let vector_concept = vector::vector_array(num_array);
    //println!("Logging of the vector, {:?}", vector_concept);
    
    let error_fn = errorhandling::error_concept();

    Ok(())
}

fn passing_reference() -> String {
    let string_in_scope = String::from("String declared in passing reference function");
    string_in_scope
}
