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

pub fn init() {
    let mut vec_data = DataContainer {
        data: vec![1, 2],
        total_elements: 2,
    };

    println!("Original vec data: {:?}", vec_data);

    vec_data.add_element_in_container(4);
    vec_data.add_element_in_container(7);

    println!("After adding element: {:?}", vec_data);

    vec_data.change_specific(22);

    println!("After changing of the specific: {:?}", vec_data);

    vec_data.remove_data_in_array(3);
    println!(
        "Removing of data in array at specific index: {:?}",
        vec_data
    );
}
