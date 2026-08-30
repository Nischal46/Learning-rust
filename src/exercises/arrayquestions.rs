struct ProductInventory<'a> {
    data: [Product<'a>; 6],
}

#[derive(Debug)]
struct Product<'a> {
    title: &'a str,
    brand: &'a str,
    price: u32,
}

fn create_product() -> Product<'static> {
    let title = "Laptop";
    let brand = "DELL";
    let price = 68000;

    Product {
        title,
        brand,
        price,
    }
}

pub fn init() {
    let product_create = create_product();
    println!("Logging of the product create: {:?}", product_create);
}
