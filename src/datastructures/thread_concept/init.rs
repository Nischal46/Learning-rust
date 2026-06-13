use std::thread;
use std::time::Duration;

fn main_thread_block() {
    println!("Please. file start to download ");

    for i in 1..=10 {
        println!("Downloading => ...... {} %", i * 10);
        thread::sleep(Duration::from_secs(1));
    }
}

pub fn init() {
    println!(
        "------------------------------This is for demonstrating of thread concept ------------------------------"
    );

    let make_async = thread::spawn(|| {
        main_thread_block();
        println!("Download complete ......... ");
    });

    //join makes synchronous and make thread to wait
    make_async.join().unwrap();

    println!("Finish task............");
}
