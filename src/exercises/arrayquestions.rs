#[derive(Debug)]
struct ProductDetails<'a> {
    title: &'a str,
    brand: &'a str,
    price: u32,
}

#[derive(Debug)]
struct ArrayContainer<'a> {
    data: [ProductDetails<'a>; 5],
}

impl<'a> ArrayContainer<'a> {
    fn sorting_by_highest_to_lowest_pricing(&mut self) {
        let total_length = self.data.len();
        for i in 0..total_length {
            for j in 0..total_length - 1 - i {
                if self.data[j].price < self.data[j + 1].price {
                    self.data.swap(j + 1, j);
                }
            }
        }

        println!("After sorting highest to lowest: {:?}", self.data);
    }

    fn sorting_by_lowest_to_highest_pricing(&mut self) {
        let total_length = self.data.len();
        for i in 0..total_length {
            for j in 0..total_length - 1 - i {
                if self.data[j].price > self.data[j + 1].price {
                    self.data.swap(j + 1, j);
                }
            }
        }

        println!("After sorting lowest to highest: {:?}", self.data);
    }

    fn reverse_array_without_extra_space(&mut self) {
        for i in self.data.iter().rev() {
            println!("{:?}", i);
        }
    }

    fn expensive_items(&self) {
        let mut extract_expensive_items = vec![];

        for item in self.data.iter() {
            if item.price > 15000 {
                extract_expensive_items.push(item);
            }
        }

        println!("Extracting expensive items: {:?}", extract_expensive_items);
    }

    fn total_expenses_of_item(&self) {
        let mut total = 0;

        for item in self.data.iter() {
            total += item.price;
        }

        println!("Finding total cost of inventory of products: {}", total);
    }
}

pub fn init() {
    println!("This is what we solve array related questions thoroughly");
    let mut array_container = ArrayContainer {
        data: [
            ProductDetails {
                title: "Laptop",
                brand: "DELL",
                price: 68000,
            },
            ProductDetails {
                title: "Monitor",
                brand: "BENQ",
                price: 20000,
            },
            ProductDetails {
                title: "Mouse",
                brand: "Fantech",
                price: 1500,
            },
            ProductDetails {
                title: "Keyboard",
                brand: "Eyoso",
                price: 5000,
            },
            ProductDetails {
                title: "Cooler",
                brand: "Nothing",
                price: 2200,
            },
        ],
    };

    println!("Logging of the array container: {:#?}", array_container);

    array_container.reverse_array_without_extra_space();
    array_container.sorting_by_highest_to_lowest_pricing();
    array_container.sorting_by_lowest_to_highest_pricing();
    array_container.expensive_items();
    array_container.total_expenses_of_item();
}
