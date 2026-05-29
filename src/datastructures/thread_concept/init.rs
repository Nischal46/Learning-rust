use std::thread;
use std::time::Duration;

fn main_thread_block() {
    thread::sleep(Duration::from_secs(6));
}

pub fn init() {
    println!(
        "------------------------------This is for demonstrating of thread concept ------------------------------"
    );

    let make_async = thread::spawn(|| {
        main_thread_block();
        println!("Finally finished......... after 6 sec");
    });

    //join makes synchronous and make thread to wait
    make_async.join().unwrap();

    println!("This run first due to asynchronaus nature.....");
}
