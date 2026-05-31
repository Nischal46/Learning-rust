use std::any::type_name;

#[derive(Debug, Clone, Copy)]
struct Line<'a> {
    name: &'a str,
    purpose: &'a str,
}

#[derive(Debug)]
struct LinearQueue<'a> {
    data: Vec<Option<Line<'a>>>,
    front: usize,
    rear: usize,
    capacity: usize,
    count: usize,
}

impl<'a> LinearQueue<'a> {
    fn new(inp: i32) -> Self {
        let alloc_size = inp as usize;
        LinearQueue {
            data: vec![None; alloc_size],
            front: 0,
            rear: 0,
            capacity: alloc_size,
            count: 0,
        }
    }

    fn is_full(&self) -> bool {
        if self.capacity == self.count {
            return true;
        }

        return false;
    }

    fn add_rear(&mut self, data: Line<'a>) {
        if self.is_full() {
            println!("Queue is full.......");
            return;
        }

        self.data[self.rear] = Some(data);
        self.rear += 1;
        self.count += 1;
    }

    fn is_empty(&self) -> bool {
        if self.count == 0 {
            return true;
        }
        return false;
    }

    fn remove_front(&mut self) {
        if self.is_empty() {
            println!("Right now queue is empty......");
            return;
        }

        self.data[self.front] = None;
        self.front += 1;
    }
}

fn print_type_of<T>(_: T) {
    println!("{}", type_name::<T>());
}

pub fn init() {
    println!("--------Concept of Linear queue-----------");

    let mut queue_concept = LinearQueue::new(2);
    queue_concept.remove_front();
    queue_concept.add_rear(Line {
        name: "Nischal Baniya",
        purpose: "Embedded Rust",
    });

    queue_concept.add_rear(Line {
        name: "Rust dev - Nischal Baniya",
        purpose: "Security",
    });

    queue_concept.add_rear(Line {
        name: "Baniya dev",
        purpose: "Game dev",
    });
    println!("Logging of the linear queue ------- {:?}", queue_concept);

    print_type_of(&queue_concept);
    print_type_of("code daily");
    print_type_of(String::from("Lovee to code"));
}
