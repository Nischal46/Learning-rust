use std::collections::VecDeque;

pub fn init() {
    println!("This is the concept for the double ended queue");

    let mut deque_alloc = VecDeque::with_capacity(3);

    deque_alloc.push_back(23);
    deque_alloc.push_back(44);
    deque_alloc.push_front(56);

    println!("Logging of the deque -- {:?}", deque_alloc);
}
