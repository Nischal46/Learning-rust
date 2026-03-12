#[derive(Debug)]
struct Product {
    title: &'static str,
    price: std::cell::Cell<u32>,
}

pub fn struct_concept() {
    let obj = Product {
        title: "Laptop",
        price: std::cell::Cell::new(23000),
    };

    obj.price.set(45000);
    println!("Logging obj ---- {:?}", obj);
}
