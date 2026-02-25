//Trait concept

use std::fs;

pub trait Summary {
    fn Summarize(&self) -> String;
}

#[derive(Debug)]
pub struct Instrument {
    instrument_type: String,
    instrument_brand: String,
    instrument_price: i32
}

impl Summary for Instrument{
    fn Summarize(&self) -> String {
        let res = "Return from trait".to_owned();
        res
    }
}

fn main(){
    let obj = Instrument {
        instrument_type: "Guitar".to_owned(),
        instrument_brand: "Mantra".to_owned(),
        instrument_price: 7500
    };

    let res = obj.Summarize();

    println!("{:#?}", obj);

    println!("{}", res);

    let read_File = fs::read_to_string("file.txt").unwrap();
    println!("Read file text: {:#?}", read_File);
}
