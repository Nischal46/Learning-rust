#[derive(Debug)]
struct Product<'a>{
    title: &'a str,
    brand: &'a str,
    price: u64,
    stock_quantity: u8
}

impl<'a> Product<'a> {
    //associative fn similar to constructor
    //difference measure by Self

    fn new(title: &'a str, brand: &'a str, price: u64, stock_quantity: u8) -> Self {
        Self {
            title: title,
            brand: brand,
            price: price,
            stock_quantity
        }
    }

    fn is_expensive_product(&self){
        if *&self.price > 12000 {
            println!("{} is expensive product cost Rs {}", &self.title, &self.price);
        }
        else {
            println!("{} is less expensive as Rs {}", &self.title, &self.price);
        }
    }
}

fn main() {
    let mut product_array: Vec<Product> = Vec::new();
    println!("This code is written in neovim ide");
    //product_array.push("Hello");
    let obj1= Product {
        title: "Chair",
        brand: "Well known",
        price: 11500,
        stock_quantity: 1
    };

    let obj2 = Product {
        title: "Monitor",
        brand: "Benq",
        price: 20000,
        stock_quantity: 1
    };

    product_array.push(obj1);

    let constructor_object = Product::new("Laptop", "Dell", 64000, 2);
    constructor_object.is_expensive_product();
    println!("Logging of the object by constructor function: {:#?}", constructor_object);

    let constructor_object = Product::new("Mouse", "Fantech", 1500, 1);
    constructor_object.is_expensive_product();
println!("Object of constructor ---{:#?}", constructor_object);
    product_array.push(obj2);
    println!("Logging of the product array: {:#?}", product_array);

}
