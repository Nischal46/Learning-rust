pub fn init() {
    println!("This is for linear queue concept !!!!");

    let mut qu_concept = LinearQueue::new(2);
    let data = Task {
        id: 0,
        title: "Wake up early",
        is_complete: true,
    };
    qu_concept.add_element(data);

    println!("Queue full details: {:?}", qu_concept);
}

#[derive(Debug, Clone)]
struct Task<'a> {
    id: u8,
    title: &'a str,
    is_complete: bool,
}

#[derive(Debug)]
struct LinearQueue<'a> {
    data: Vec<Option<Task<'a>>>,
    front: usize,
    rear: usize,
    count: usize,
    capacity: usize,
}

impl<'a> LinearQueue<'a> {
    fn new(size: u8) -> Self {
        let alloc_size = size as usize;

        LinearQueue {
            data: vec![None; alloc_size],
            front: 0,
            rear: 0,
            count: 0,
            capacity: alloc_size,
        }
    }

    fn is_full(&self) -> bool {
        if self.capacity == self.rear {
            return true;
        } else {
            return false;
        }
    }

    fn is_empty(&self) -> bool {
        if self.count == 0 {
            return true;
        } else {
            return false;
        }
    }

    fn add_element(&mut self, data: Task<'a>) {
        if self.is_full() {
            println!("Queue is full. cannot add other data....");
            return;
        }

        self.data[self.rear] = Some(data);
        self.rear += 1;
        self.count += 1;
    }

    fn remove_element(&mut self) {
        if self.is_empty() {
            println!("Queue is currently empty..,,, Please insert data first");
            return;
        }

        self.data[self.front] = None;
        self.front += 1;
    }
}
