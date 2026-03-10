trait DetailsTrait {
    fn detials(&self) -> String;
}

#[derive(Debug)]
struct Instrument<'a> {
    title: &'a str,
    brand: &'a str,
    price: u16,
}

impl<'a> Instrument<'a> {
    fn new(title: &'a str, brand: &'a str, price: u16) -> Self {
        Instrument {
            title,
            brand,
            price,
        }
    }
}

impl DetailsTrait for Instrument<'_> {
    fn detials(&self) -> String {
        println!("This is trait fn declaration");

        let returning_string = format!(
            "Instrument: {}, Brand: {}, Price: {}",
            self.title, self.brand, self.price
        );

        returning_string
    }
}

pub fn struct_concept() {
    let ins_obj = Instrument::new("Guitar", "Mantra", 7500);

    println!("Object of instrument: {:?}", ins_obj);

    println!("this is method returning from imp: {}", ins_obj.detials());
}
