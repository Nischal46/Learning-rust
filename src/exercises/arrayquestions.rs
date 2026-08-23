#[derive(Debug)]
struct ArrayContainer {
    data: [Option<u32>; 5],
}

impl ArrayContainer {
    fn sort() {
        println!("This is sorting of the method");
    }

    fn check_array_container_full(&self) -> bool {
        if self.data.contains(&None) {
            println!("Array still have empty slot");
            false
        } else {
            println!("Array is full");
            true
        }
    }

    fn add_data_in_Array_container(&mut self, newdata: u32) {
        if self.check_array_container_full() != true {
            println!("still remaining");

            for slot in self.data.iter_mut() {
                if slot.is_none() {
                    *slot = Some(newdata);
                }
            }
        } else {
            println!("full full")
        }
    }
}

pub fn init() {
    println!("This is for solving of the exercises in array for rust");

    let mut array_container = ArrayContainer {
        data: [Some(1), Some(2), Some(3), Some(4), None],
    };

    //array_container.data[4] = Some(4);

    println!("Logging of th array container: {:?}", array_container.data);

    array_container.check_array_container_full();
    array_container.add_data_in_Array_container(45);

    println!(
        "After mutating new data in container --- {:?}",
        array_container.data
    );
}
