use std::{thread, time::Duration};

pub fn init() {
    //dissecting of the thread concept

    println!("Thread concept in details");

    println!("Thread blocks for 3 sec");
    thread::sleep(Duration::from_secs(3));

    let async_one_thread = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("sleep for 1 sec.........");
    });

    println!("Easy thread concept knowledge");

    async_one_thread.join().expect("Failed failed");
}

