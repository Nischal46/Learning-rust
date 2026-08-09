use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn init() {
    let mutex_brand = Arc::new(Mutex::new(3));

    let mut box_pointer = Arc::new(Box::new(27));
    let cloning_box_pointer = Arc::clone(&box_pointer);

    // cannot mutate becase it doesnot have unique ownership
    if let Some(box_ref) = Arc::get_mut(&mut box_pointer) {
        **box_ref = 67;
    }

    passing_box_pointer_reference(cloning_box_pointer);
    println!("Logging of box pointer: {}", box_pointer);

    println!(
        "Before changing mutex val. original val: {:?}",
        mutex_brand.lock().unwrap()
    );
    block_mutate_when_mutex_not_release(Arc::clone(&mutex_brand));

    println!(
        "After changing mutex value. mutate value: {:?}",
        mutex_brand.lock().unwrap()
    );
}

fn passing_box_pointer_reference(mut box_value: Arc<Box<i32>>) {
    **Arc::make_mut(&mut box_value) = 100;
    println!("Decoding the box value inside this fn scope: {}", box_value);
}

fn block_mutate_when_mutex_not_release(mutext_change_val: Arc<Mutex<i32>>) {
    let mut count = 1;
    let mut data = mutext_change_val.lock().unwrap();
    loop {
        if count != 5 {
            println!("Count: {}", count);
            thread::sleep(Duration::from_secs(1));
            count += 1;
        } else {
            println!("Count reached");
            *data = 12;
            break;
        }
    }
}
