//NOTE using vec data type because it is dynamically allocated

#[derive(Debug)]
struct DataContainer {
    data: Vec<u32>,
    total_elements: usize,
}

impl DataContainer {
    fn add_element_in_container(&mut self, element: u32) {
        self.data.push(element);
        self.total_elements += 1;
    }

    fn change_specific(&mut self, element: u32) {
        if let Some(value) = self.data.get_mut(1) {
            *value = element;
        }
    }

    fn remove_data_in_array(&mut self, index: usize) {
        self.data.remove(index);
        self.total_elements -= 1;
    }
}

#[derive(Debug, Clone)]
struct ProductDetail {
    id: u32,
    title: String,
    brand: String,
    price: i32,
}

#[derive(Debug)]
struct ProductDataContainer {
    data: Vec<ProductDetail>,
    total_data: usize,
}

impl ProductDataContainer {
    fn new() -> Self {
        Self {
            data: vec![],
            total_data: 0,
        }
    }

    fn add_object_in_container(&mut self, title: String, brand: String, price: i32) {
        self.total_data += 1;
        self.data.push(ProductDetail {
            id: self.total_data as u32,
            title: title.to_string(),
            brand: brand.to_string(),
            price: price,
        });
    }

    fn update_object_in_container(&mut self, id: u32) {
        if let Some(item) = self.data.iter_mut().find(|item| item.id == id) {
            item.title = "Laptop Boosted".to_string()
        }
    }

    fn find_object_index_in_container(&self, title: String) -> Option<ProductDetail> {
        self.data.iter().find(|item| item.title == title).cloned()
    }
}

pub fn init() {
    let mut vec_data = DataContainer {
        data: vec![1, 2],
        total_elements: 2,
    };

    // println!("Original vec data: {:?}", vec_data);
    //
    // vec_data.add_element_in_container(4);
    // vec_data.add_element_in_container(7);
    //
    // println!("After adding element: {:?}", vec_data);
    //
    // vec_data.change_specific(22);
    //
    // println!("After changing of the specific: {:?}", vec_data);

    // vec_data.remove_data_in_array(3);
    // println!(
    //     "Removing of data in array at specific index: {:?}",
    //     vec_data
    // );

    let mut product_vec_data = ProductDataContainer::new();
    product_vec_data.add_object_in_container("Laptop".to_string(), "DELL".to_string(), 68000);
    product_vec_data.add_object_in_container("Keyboard".to_string(), "Eyoso".to_string(), 5000);

    println!(
        "Logging of the vec data type of the product: {:#?}",
        product_vec_data
    );

    match product_vec_data.find_object_index_in_container("Keyboard".to_string()) {
        Some(data) => println!(
            "Finding specific data from vec product container: {:?}",
            data
        ),
        None => println!("Opps. donot find the data you search for"),
    }

    product_vec_data.update_object_in_container(1);

    println!("[Updating] Logging vec: {:#?}", product_vec_data);
}
