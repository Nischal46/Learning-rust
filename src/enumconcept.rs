pub fn enum_concept() {
    let result = internal_enum();
    println!("Logging choosing of result enum -- {:?}", result);
}

fn internal_enum() -> Option<Product> {
    println!("This is internal enum");
    let choose_product: Option<Product> = None;
    //choose_product = Some(Product::Laptop);
    return choose_product;
}

#[derive(Debug)]
enum Product {
    Laptop,
    Keyboard,
    Mouse,
}
