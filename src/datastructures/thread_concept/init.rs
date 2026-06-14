use std::thread;
use std::time::Duration;

fn main_thread_block() {
    println!("Please. file start to download ");

    for i in 1..=10 {
        println!("Downloading => ...... {} %", i * 10);
        thread::sleep(Duration::from_secs(1));
    }
}

fn next_thread() {
    for i in 1..=100 {
        println!("Logging from thread internal ... {} %", i);
        thread::sleep(Duration::from_millis(300));
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

    for i in 1..=10 {
        println!("Initializing of ui => .......{}%", i * 10);
        thread::sleep(Duration::from_secs(1));
    }

    let another_async = thread::spawn(|| {
        next_thread();
        println!(
            "Finished of the internal thread -------------------------------------------------------------"
        );
    });

    //join makes synchronous and make thread to wait
    make_async.join().unwrap();
    another_async
        .join()
        .expect("Failed to run the internal thread ...");

    println!("Finish task............");
}
