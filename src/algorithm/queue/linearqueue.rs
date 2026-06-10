//NOTE: Linear queue concept

#[derive(Debug)]
struct Task<'a> {
    id: u8,
    title: &'a str,
    is_complete: bool,
}

#[derive(Debug)]
struct Queue<'a> {
    data: Vec<Option<Task<'a>>>,
    front: usize,
    rear: usize,
    capacity: usize,
    count: usize,
}

impl<'a> Queue<'a> {
    fn new(determine_size: u8) -> Self {
        let alloc_size = determine_size as usize;
        Queue {
            data: vec![None; alloc_size],
            front: 0,
            rear: 0,
            capacity: alloc_size,
            count: 0,
        }
    }
}

pub fn init() {
    println!("Linear Queue concept................");

    let queue_dec = Queue::new(3);
}
