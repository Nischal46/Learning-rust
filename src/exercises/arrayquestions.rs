#[derive(Debug)]
struct ArrayData {
    data: [u32; 5],
}

impl ArrayData {
    fn find_highest_data_in_array(&self) -> u32 {
        let mut highest = 0;
        for &num in self.data.iter() {
            if highest < num {
                highest = num;
            }
        }
        return highest;
    }

    fn check_is_sorted_or_not(&self) -> bool {
        let mut starting_var = 0;
        let mut result = true;
        for &num in self.data.iter() {
            if num > starting_var {
                starting_var = num;
                result = true;
            } else {
                result = false;
            }
        }

        return result;
    }

    fn find_index_with_for_loop(&self, element_to_find: u32) -> Option<usize> {
        for (index, &num) in self.data.iter().enumerate() {
            if element_to_find == num {
                return Some(index);
            }
        }
        None
    }

    fn find_index_with_position_built_in_method(&self, element_to_find: u32) -> Option<usize> {
        self.data.iter().position(|&num| num == element_to_find)
    }
}

pub fn init() {
    println!("-------- Array & its related question ------------");

    let passing_array_data_one = ArrayData {
        //data: [12, 56, 23, 43, 8],
        data: [1, 2, 4, 5, 7],
    };

    let highest_number = passing_array_data_one.find_highest_data_in_array();
    println!("Logging highest element in array: {}", highest_number);

    let arraysorted_or_not = passing_array_data_one.check_is_sorted_or_not();
    println!("Logging array sorted or not: {}", arraysorted_or_not);

    let element_to_find = passing_array_data_one.find_index_with_for_loop(57);
    println!(
        "Logging index of element data position in array: {:?}",
        element_to_find
    );

    let element_to_find_by_builtin =
        passing_array_data_one.find_index_with_position_built_in_method(12);

    match element_to_find_by_builtin {
        Some(ind) => println!("Yeah find that data in index: {}", ind),
        None => println!("Sorry dude. element not found"),
    }
}
