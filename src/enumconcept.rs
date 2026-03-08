pub fn enum_concept() {
    let result = internal_enum();
    println!("Logging choosing of result enum -- {:?}", result);

    let instrument_result = choosing_instrument_enum();
    println!("choosing_instrument_enum: {:?}", instrument_result);

    for n in 1..=3 {
        println!("iteration loop: {}", n);
        if n == 1 {
            println!("if 1 then {:?}", Instrument::Ukulele);
        } else if n == 2 {
            println!("if 2 then {:?}", Instrument::Flute);
        } else {
            println!("if 3 then any");
        }
    }
}

fn internal_enum() -> Option<Product> {
    println!("This is internal enum");
    let choose_product: Option<Product> = None;
    //choose_product = Some(Product::Laptop);
    return choose_product;
}

fn choosing_instrument_enum() -> Option<Instrument> {
    let mut choose_instrument: Option<Instrument> = None;
    choose_instrument = Some(Instrument::Guitar);
    return choose_instrument;
}

#[derive(Debug)]
enum Product {
    Laptop,
    Keyboard,
    Mouse,
}

#[derive(Debug)]
enum Instrument {
    Guitar,
    Ukulele,
    Flute,
}
