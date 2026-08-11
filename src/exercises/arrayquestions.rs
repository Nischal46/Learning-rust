#[derive(Debug)]
struct ArrayStructure {
    data: [u32; 5],
}

impl ArrayStructure {
    // NOTE find highest element in array
    fn find_highest_number_in_array(&self) {
        let mut highest: u32 = 0;
        for &num in &self.data {
            if highest < num {
                highest = num;
            }
        }

        println!("{} is highest among {:?}", highest, &self.data);
    }

    // NOTE search element in array
    fn search_element_in_array(&self, element_to_find: u32) -> Option<usize> {
        // for (index, &num) in self.data.iter().enumerate() {
        //     if num == element_to_find {
        //         println!("{} found at: {}", num, index);
        //         break;
        //     } else {
        //         println!("{} not found", num);
        //     }
        // }

        self.data.iter().position(|&num| num == element_to_find)
    }
}

pub fn init() {
    println!("Array related questions");

    let arraydata = ArrayStructure {
        data: [1, 2, 10, 4, 5],
    };

    arraydata.find_highest_number_in_array();
    arraydata.search_element_in_array(10);

    match arraydata.search_element_in_array(4) {
        Some(index) => println!("Element found at {}", index),
        None => println!("Element not found"),
    }

    println!("Logging of the array data: {:?}", arraydata);
}
