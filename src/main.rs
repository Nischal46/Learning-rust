fn main() {
    println!("This is the main entry of the app");

    let product = Product::new("Laptop", 68000);

    let result = product.discount();
    println!("Result of discount price of the product: {}", result);
}

struct Product<'a> {
    title: &'a str,
    price: i32,
}

impl<'a> Product<'a> {
    //NOTE: mark as constructor fn
    fn new(title: &'a str, price: i32) -> Self {
        Self { title, price }
    }

    fn discount(&self) -> String {
        let discount_amount = (self.price / 100) * 10;
        let discount_price = self.price - discount_amount;

        format!("Discount price: {}", discount_price)
    }
}
