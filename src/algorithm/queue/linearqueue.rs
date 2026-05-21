#[derive(Debug)]
struct Queue {
    data: Vec<Option<i32>>,
    front: usize,
    rear: usize,
    capacity: usize,
    size: usize,
}

impl Queue {
    fn new(allocSize: i32) -> Self {
        let sizeDeclare = allocSize as usize;
        Queue {
            data: vec![None; sizeDeclare],
            front: 0,
            rear: 0,
            capacity: sizeDeclare,
            size: 0,
        }
    }

    fn is_queue_full(&self) -> bool {
        self.capacity == self.size
    }

    fn is_queue_empty(&self) -> bool {
        self.size == 0
    }

    fn add_element_in_queue(&mut self, elem: i32) {
        if self.is_queue_full() {
            println!("Queue is full. you cannot insert element");
            return;
        }

        self.data[self.rear] = Some(elem);
        self.rear += 1;
        self.size += 1;
    }

    fn remove_element_in_queue(&mut self) {
        if self.is_queue_empty() {
            println!("Queue is empty..");
            return;
        }

        self.data[self.front] = None;
        self.front += 1;
        self.size -= 1;
    }
}

pub fn init() {
    println!("This is for linear queue concept");

    let mut single_queue_concept = Queue::new(3);
    single_queue_concept.remove_element_in_queue();
    single_queue_concept.add_element_in_queue(23);
    single_queue_concept.add_element_in_queue(56);
    single_queue_concept.add_element_in_queue(67);
    single_queue_concept.add_element_in_queue(79);
    single_queue_concept.remove_element_in_queue();
    single_queue_concept.remove_element_in_queue();

    println!(
        "Logging of Single ended queue :-> {:?}",
        single_queue_concept
    );
}
