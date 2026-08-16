// NOTE vec related problems

#[derive(Debug)]
struct Product<'a> {
    id: i32,
    title: &'a str,
    price: i32,
    brand: &'a str,
}

#[derive(Debug)]
struct ProductInventory<'a> {
    data: Vec<Product<'a>>,
    total_elements: usize,
}

impl<'a> ProductInventory<'a> {
    fn new() -> Self {
        Self {
            data: vec![],
            total_elements: 0,
        }
    }

    fn add_item_in_inventory(&mut self, added_item: Product<'a>) {
        self.total_elements += 1;
        self.data.push(added_item);
    }

    fn update_item_in_inventory(&mut self, updated_item: Product<'a>) {}

    fn get_specific_item_from_inventory(&self, id: i32) {}
}

pub fn init() {
    println!("======= Vec related exercises ========");

    let mut product_container_inventory = ProductInventory::new();
    product_container_inventory.add_item_in_inventory(Product {
        id: 1,
        title: "Black Sleek Laptop",
        price: 68000,
        brand: "DELL",
    });

    product_container_inventory.add_item_in_inventory(Product {
        id: 2,
        title: "Mechanical keyboard",
        price: 5000,
        brand: "Eyooso",
    });

    println!("Logging of vec data type ---------");
    println!("{:#?}", product_container_inventory);
}
