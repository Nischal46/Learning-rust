pub fn init() {
    println!("--------------Single ended queue-------------");
    let mut queue_first_batch = Queue::new(3);
    queue_first_batch.add(23);
    queue_first_batch.add(56);
    queue_first_batch.add(87);
    queue_first_batch.remove();

    println!(
        "Logging of single ended queue -------- {:?}",
        queue_first_batch
    );
}

#[derive(Debug)]
struct Queue {
    data: Vec<Option<i32>>,
    front: usize,
    rear: usize,
    capacity: usize,
    count: usize,
}

impl Queue {
    fn new(inp: i32) -> Self {
        let alloc_size = inp as usize;
        Self {
            data: vec![None; alloc_size],
            front: 0,
            rear: 0,
            capacity: alloc_size,
            count: 0,
        }
    }

    fn is_full(&self) -> bool {
        self.count == self.capacity
    }

    fn add(&mut self, elem: i32) {
        if self.is_full() {
            println!("Queue is empty for today.");
            return;
        }

        self.data[self.rear] = Some(elem);
        self.rear += 1;
        self.count += 1;
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn remove(&mut self) {
        if self.is_empty() {
            println!("Queue is empty .....");
            return;
        }

        self.data[self.front] = None;
        self.front += 1;
    }
}
