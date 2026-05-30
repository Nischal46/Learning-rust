pub fn init() {
    println!("This is the concept of the linear queue");
}

struct LinearQueue {
    data: Vec<Option<i8>>,
    front: usize,
    back: usize,
    capacity: usize,
    count: usize,
}

pub trait CheckMargin {
    fn new(alloc_Size: i8) -> Self;
    fn check_input_margin(inp: i8);
}

impl CheckMargin for LinearQueue {
    fn new(alloc_size: i8) -> Self {
        let size_determined = alloc_size as usize;
        LinearQueue {
            data: vec![None; size_determined],
            front: 0,
            back: 0,
            capacity: size_determined,
            count: 0,
        }
    }

    fn check_input_margin(inp: i8) {}
}
