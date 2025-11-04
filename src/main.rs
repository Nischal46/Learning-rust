#[derive(Debug)]
struct ProductDetails {
    product_type: String,
    product_brand: String,
    is_available: bool
}

trait ProductTrait {
    fn greet(&self) -> String;
}

impl ProductTrait for ProductDetails {
    fn greet(&self) -> String {
        format!("Congratulations. you had won {:#?}", &self.product_brand)
    }
}

fn main() {
    use ProductTrait; // Bring trait into scope
    
    let product1 = ProductDetails {
        product_type: "Laptop".to_string(),
        product_brand: "DELL".to_string(),
        is_available: true
    };

    println!("{:#?}", product1);
    println!("{:#?}", product1.greet());

}