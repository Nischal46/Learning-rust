//today concept for Cell that can change the object without making variable mut

use std::cell::Cell;

#[derive(Debug)]
struct OSTask<'a> {
    task: &'a str,
    queue_priority: Cell<u8>,
}

impl<'a> OSTask<'a> {
    fn new(task_inp: &'a str, task_priority: u8) -> Self {
        Self {
            task: task_inp,
            queue_priority: Cell::new(task_priority),
        }
    }

    fn change_queue_priority(&self) {
        self.queue_priority.set(2);
    }
}

fn main() {
    let task_definer = OSTask::new("Gnome runner", 1);
    println!("Before updating by cell");
    println!("Logging os task: {:?}", task_definer);

    println!("After updating by cell");
    task_definer.change_queue_priority();
    println!("Logging os task: {:?}", task_definer);
}
