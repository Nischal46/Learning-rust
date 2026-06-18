use std::{thread, time::Duration};

pub fn init() {
    println!("This is a thread concept ....");

    let thread_one = thread::spawn(|| {
        async_thread_one();
    });

    let thread_two = thread::spawn(|| {
        async_thread_two();
    });

    println!("Before async");
    thread::sleep(Duration::from_secs(4));
    println!("First of all it block for 4 sec before async");

    println!("Async behaviour....... start here");
    thread_one.join().expect("Issue in thread one");
    thread_two.join().expect("Issue in thread two");
}

fn async_thread_one() {
    thread::sleep(Duration::from_secs(5));
    println!("Thread release only after 5 sec");
}

fn async_thread_two() {
    thread::sleep(Duration::from_secs(3));
    println!("Thread release only after 3 sec");
}
