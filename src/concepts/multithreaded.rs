use std::{thread, time::Duration};

pub fn init() {
    // concept between threaded and the real use of mutex to prevent
    println!("Hello hello");

    println!("Thread block for .....");

    let mut count = 0;

    for i in 1..=5 {
        count = count + 1;
        println!("Countdown: {}", i);
        thread::sleep(Duration::from_secs(1));
    }

    println!("Finally thread occupied released after {} sec", count);
}

