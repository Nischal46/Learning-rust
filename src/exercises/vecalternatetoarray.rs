// NOTE vec data related exercises

#[derive(Debug)]
struct Product<'a> {
    title: &'a str,
    price: i32,
    brand: &'a str,
    manufacture_country: &'a str,
}

#[derive(Debug)]
struct ProductContainer<'a> {
    data: Vec<Product<'a>>,
    total_elements: usize,
}

impl<'a> ProductContainer<'a> {
    fn new() -> Self {
        Self {
            data: vec![],
            total_elements: 0,
        }
    }

    fn add_item(&mut self, inp: Product<'a>) {
        self.total_elements += 1;
        self.data.push(inp);
    }

    fn update_item(&mut self, title: &str) {
        if let Some(item) = self.data.iter_mut().find(|item| item.title == title) {
            item.title = "God level laptop";
        }
    }
}

pub fn init() {
    println!("---solving vec related exercise for dynamic memory management------");

    let mut product_container = ProductContainer::new();
    product_container.add_item(Product {
        title: "Laptop",
        price: 68000,
        brand: "DELL",
        manufacture_country: "China",
    });

    println!("Logging of the vec data by adding lifetime varible not string......");
    println!("{:#?}", product_container);

    println!("trying to update lifetime variable also");
    product_container.update_item("Laptop");
    println!("{:#?}", product_container);
}
