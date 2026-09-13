struct ArrayContainer {
    data: [Option<i32>; 5]
}

impl ArrayContainer {
    fn new() -> Self {
        Self { data: [None; 5] }
    }

    fn extract_present_values_in_array(&self) -> Vec<i32> {
        self.data.iter().filter_map(|&x| x).collect()
    }
}

pub fn init() {
    let arr = [1, 4, 3];
    println!("{:?}", arr);

    let mut arr_container_declared = ArrayContainer::new();
    arr_container_declared.data[2] = Some(10);
    println!("{:?}", arr_container_declared.data);
    println!("fetching of the arr index 3: {:?}", arr_container_declared.data[2].unwrap());

    let present_values = arr_container_declared.extract_present_values_in_array();
    println!("Present values in the array: {:?}", present_values);
}
