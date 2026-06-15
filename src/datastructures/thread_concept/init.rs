use std::{thread, time::Duration};

pub fn init() {
    println!("Thread concept ...");

    let async_thread_one = thread::spawn(|| {
        thread_one();
    });

    let async_thread_two = thread::spawn(|| {
        thread_two();
    });

    println!("Async behaviour ------- ");

    async_thread_one.join().unwrap();
    async_thread_two.join().unwrap();
}

fn thread_one() {
    thread::sleep(Duration::from_secs(4));
    println!("Print log after 4 sec");
}

fn thread_two() {
    thread::sleep(Duration::from_secs(2));
    println!("Print log after 2 sec");
}
