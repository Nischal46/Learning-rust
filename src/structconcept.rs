trait Details {
    fn details(&self) {}
    fn manufacturer(&self) {}
}

#[derive(Debug)]
struct Instrument<'a> {
    title: &'a str,
}

impl Details for Instrument<'_> {
    fn details(&self) {
        println!(
            "Impl block with traits and instrument title: {}",
            self.title
        )
    }
}

#[derive(Debug)]
struct Electronic {
    product: String,
    price: u32,
}

impl Details for Electronic {
    fn manufacturer(&self) {
        println!(
            "Manufacturer: Nischal, Product: {} & Price: {}",
            self.product, self.price
        );
    }
}

pub fn struct_concept() {
    let obj_1 = Instrument { title: "Guitar" };
    println!("Instrument Object: {:?}", obj_1);
    obj_1.details();

    let obj_2 = Electronic {
        product: "Laptop".to_owned(),
        price: 68000,
    };

    obj_2.manufacturer();

    println!("Electronic object: {:?}", obj_2);
}
