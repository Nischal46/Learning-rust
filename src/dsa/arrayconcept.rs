#[derive(Debug)]
struct Arraycontainer {
    data: [i32; 5],
}

impl Arraycontainer {
    fn find_highest_element(&self) -> Option<&i32> {
        let mut highest_element = &self.data[0];
        for i in self.data.iter() {
            println!("Logging value inside of for loop {}", i);

            if i > highest_element {
                highest_element = i;
            }
        }

        Some(highest_element)
    }
}

pub fn init() {
    // here we will solve array related questions serially

    let arr_container = Arraycontainer {
        data: [12, 43, 54, 65, 76],
    };

    let second_arr_container = Arraycontainer {
        data: [32, 12, 54, 65, 76],
    };

    println!("Logging of the arr_container: {:?}", arr_container);
    println!(
        "Logging of the array second container {:#?}",
        second_arr_container
    );

    let finding_highest_element_in_array = second_arr_container.find_highest_element();

    println!(
        "After finfding of the highest element in the array: {}",
        finding_highest_element_in_array.unwrap()
    );
}
