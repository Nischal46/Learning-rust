#[derive(Debug)]
struct Queue {
    data: [i32; 8],
    front: usize,
    back: usize,
    capacity: usize,
}

impl Queue {
    // NOTE: like constructor
    fn new() -> Self {
        Queue {
            data: [0; 8],
            front: 0,
            back: 0,
            capacity: 0,
        }
    }

    fn enqueue(&mut self, item: i32) {
        self.data.push(item);
    }

    fn dequeue(&mut self) {
        self.data.pop();
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub fn init() {
    println!("This is for linear queue concept");

    let mut queue_object = Queue::new();
    queue_object.enqueue(23);
    queue_object.enqueue(45);
    queue_object.enqueue(56);
    queue_object.dequeue();

    println!("Logging of the queue object ------ {:?}", queue_object);
}
