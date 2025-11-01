trait Product {
    fn product_title(&self) -> &str;
}

#[derive(Debug)]
struct Laptop {
    title: String
}

impl Product for Laptop {
    fn product_title(&self) -> &str {
        &self.title
    }
}
//reading of trait 
fn main() {
   //trait is like interface like we define interface in ts

   let product1 = Laptop {
    title: "Dell Laptop".to_string()
   };

   println!("{:#?}", product1.product_title());
}

// we can make it reusable in other struct form complex data type



