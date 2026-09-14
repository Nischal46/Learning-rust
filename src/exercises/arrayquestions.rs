use std::time::Instant;

struct ArrayContainer {
    data: [Option<i32>; 10]
}

impl ArrayContainer {
    fn new() -> Self {
        Self { data: [None; 10] }
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
    // arr_container_declared.data[9998] = Some(23);
    println!("{:?}", arr_container_declared.data);
    println!("fetching of the arr index 3: {:?}", arr_container_declared.data[2].unwrap());
    // println!("fetching of the arr index 9999: {:?}", arr_container_declared.data[9998].unwrap());

    let present_values = arr_container_declared.extract_present_values_in_array();
    println!("Present values in the array: {:?}", present_values);

    let start = Instant::now();

    for i in 0i64..1000000000 {
    }
    
    let end = Instant::now();
    println!("Time taken: {:?}", end - start);
}
