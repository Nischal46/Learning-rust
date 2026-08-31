#[derive(Debug)]
struct Product {
    title: String,
    brand: String,
}

fn create_product() -> Product {
    let title = String::from("Laptop");
    let brand = String::from("BENQ");

    let product_details = Product {
        title: title,
        brand: brand,
    };

    product_details
}

#[derive(Debug)]
struct ProductStr<'a> {
    title: &'a str,
    brand: &'a str,
}

fn create_product_Str() -> ProductStr<'static> {
    let title = "Laptop";
    let brand = "Nothing";

    let product_str = ProductStr { title, brand };

    product_str
}

pub fn init() {
    let response_from_product = create_product();
    println!("Response returning from : {:?}", create_product());

    let proct_str = create_product_Str();
    println!("Response of str: {:?}", proct_str);
}
