use std::collections::VecDeque;

pub fn deque_concept() {
    println!("This is the concept for the double ended queue");

    let mut deque_alloc = VecDeque::with_capacity(3);

    deque_alloc.push_back(23);
    deque_alloc.push_back(44);
    deque_alloc.push_front(56);

    println!("Before removing from deque");
    println!("Logging of the deque -- {:?}", deque_alloc);

    println!("After removing from deque");
    deque_alloc.pop_back();
    deque_alloc.pop_front();

    print!("Logging of the the deque in final --- {:?}", deque_alloc);
}
