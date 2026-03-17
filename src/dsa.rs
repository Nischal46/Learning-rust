use std::collections::{LinkedList, linked_list};

pub fn dsa() {
    let mut linkedlist_concept = LinkedList::new();
    linkedlist_concept.push_back(22);

    linkedlist_concept.push_front(23);
    linkedlist_concept.push_back(12);

    println!("Printing of linked list: {:?}", linkedlist_concept);

    println!("Length of the linked list: {}", linkedlist_concept.len());

    insert_at_position(&mut linkedlist_concept, 1, 106);

    println!(
        "After adding in specific position: {:?}",
        linkedlist_concept
    );
}

fn insert_at_position<T>(list: &mut LinkedList<T>, position: usize, value: T) {
    if position == 0 {
        list.push_front(value)
    } else if position >= list.len() {
        list.push_back(value);
    } else {
        let mut tail = list.split_off(position);
        list.push_back(value);
        list.append(&mut tail);
    }
}
