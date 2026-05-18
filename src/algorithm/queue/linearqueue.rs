#[derive(Debug)]
struct Queue {
    data: Vec<i32>,
    front: usize,
    rear: usize,
    capacity: usize,
    size: usize,
}

impl Queue {
    fn new(alloc: i32) -> Self {
        let alloc_capacity = alloc as usize;
        Queue {
            data: vec![0; alloc_capacity],
            front: 0,
            rear: 0,
            capacity: alloc_capacity,
            size: 0,
        }
    }

    fn add_elem_in_queue(&mut self, elem: i32) {
        self.data[self.rear] = elem;
        self.rear += 1;
        self.size += 1;
    }
}

pub fn init() {
    println!("This is for linear queue concept");

    let mut queue_concept = Queue::new(4);
    queue_concept.add_elem_in_queue(34);
    queue_concept.add_elem_in_queue(56);
    queue_concept.add_elem_in_queue(67);
    queue_concept.add_elem_in_queue(78);
    queue_concept.add_elem_in_queue(89);

    println!("Logging of the queue: {:?}", queue_concept);
}
