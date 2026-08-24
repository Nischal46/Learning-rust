#[derive(Debug)]
struct ArrayContainer {
    data: [Option<u32>; 5],
}

impl ArrayContainer {
    fn sideline_zero_At_last(&mut self) {}

    fn sorting_data(&mut self) {
        for i in 0..self.data.len() {
            for j in (0..self.data.len() - 1 - i) {
                if self.data[j].unwrap() > self.data[j + 1].unwrap() {
                    self.data.swap(j, j + 1);
                }
            }
        }
    }

    fn maximum_element_in_array(&self) {
        let mut maximum_element = &self.data[0];
        for i in self.data.iter() {
            if maximum_element.unwrap() < i.unwrap() {
                maximum_element = i
            }
        }

        println!("maximum element: {}", maximum_element.unwrap());
    }

    fn minimum_element_in_array(&self) {
        let mut minimun_element = &self.data[0];

        for i in self.data.iter() {
            if minimun_element.unwrap() > i.unwrap() {
                minimun_element = i;
            }
        }

        println!("minimum element: {}", minimun_element.unwrap());
    }
}

pub fn init() {
    let mut array_container = ArrayContainer {
        data: [Some(2), Some(5), Some(1), Some(4), Some(3)],
    };

    println!("Logging of the array: {:?}", array_container);
    array_container.sorting_data();

    println!("Logging after sorting: {:?}", array_container);
    array_container.maximum_element_in_array();
    array_container.minimum_element_in_array();
}
