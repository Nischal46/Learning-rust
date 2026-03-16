use std::fs::File;
use std::io::prelude::*;

pub fn minigrep() {
    let mut f = File::open("afile.txt").expect("File not found");
    let mut contents = String::new();

    let mut a3 = File::open("a3.txt").expect("a3 file not found");

    let mut data = String::new();

    f.read_to_string(&mut contents)
        .expect("Something went wrong while reading txt file");

    a3.read_to_string(&mut data)
        .expect("Failed to read a3 content");

    println!("Found these txt: \n{}", contents);

    println!("A3 content data: \n{}", data);
}
