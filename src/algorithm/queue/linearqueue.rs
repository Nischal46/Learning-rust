#[derive(Debug)]
struct Queue {
    data: Vec<i32>, // Changed from array to Vec for dynamic sizing
    front: usize,
    rear: usize,
    capacity: usize,
}

impl Queue {
    fn new(capacity: i32) -> Self {
        let capacity_usize = capacity as usize;
        Queue {
            data: vec![0; capacity_usize], // Create vector with initial capacity
            front: 0,
            rear: 0,
            capacity: capacity_usize,
        }
    }

    fn enqueue(&mut self, elem: i32) -> bool {
        if self.is_full() {
            println!("Queue is full! Cannot enqueue {}", elem);
            return false;
        }

        self.data[self.rear] = elem;
        self.rear = (self.rear + 1) % self.capacity;
        true
    }

    fn dequeue(&mut self) -> Option<i32> {
        if self.is_empty() {
            println!("Queue is empty! Cannot dequeue");
            return None;
        }

        let value = self.data[self.front];
        self.front = (self.front + 1) % self.capacity;
        Some(value)
    }

    fn is_empty(&self) -> bool {
        self.front == self.rear
    }

    fn is_full(&self) -> bool {
        (self.rear + 1) % self.capacity == self.front
    }

    fn peek(&self) -> Option<&i32> {
        if self.is_empty() {
            None
        } else {
            Some(&self.data[self.front])
        }
    }
}

pub fn init() {
    println!("This is for linear queue concept");

    let mut queue_concept = Queue::new(4);
    queue_concept.enqueue(23);
    queue_concept.enqueue(34);
    queue_concept.enqueue(45);

    println!("After enqueuing: {:?}", queue_concept);

    // Demonstrate dequeue
    if let Some(value) = queue_concept.dequeue() {
        println!("Dequeued: {}", value);
    }

    println!("After dequeuing: {:?}", queue_concept);
}
