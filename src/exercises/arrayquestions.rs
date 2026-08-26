#[derive(Debug)]
struct Product<'a> {
    title: &'a str,
    brand: &'a str,
    price: u32,
}

#[derive(Debug)]
struct ShopInventory<'a> {
    data: [Option<Product<'a>>; 5],
}

pub fn init() {
    println!("----- Array Questions -------");

    let inventory = ShopInventory {
        data: [
            Some(Product {
                title: "Mouse",
                price: 1500,
                brand: "Fantech",
            }),
            Some(Product {
                title: "Laptop",
                brand: "DELL",
                price: 68000,
            }),
            Some(Product {
                title: "Monitor",
                brand: "BENQ",
                price: 20000,
            }),
            Some(Product {
                title: "HDMI converter",
                brand: "HP",
                price: 500,
            }),
            None,
        ],
    };

    println!("Logging of the inventory: {:#?}", inventory);
}
