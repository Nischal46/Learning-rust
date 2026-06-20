use std::{thread, time::Duration};

pub fn init() {
    //NOTE: Concept about the thread

    let v = vec!["gitlab", "github", "git"];

    let sub_thread_one = thread::spawn(|| {
        for i in 1..10 {
            println!("-> Running --secondary thread-- {} times", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let sub_thread_two = thread::spawn(|| {
        for i in 1..20 {
            println!("-> Running --tertiary thread-- {} times", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    let transfering_vec_closure = thread::spawn(move || {
        println!("Heres a vector value expecting -- {:?}.", v);
    });

    // NOTE: got error as v already move to closure in new thread
    //println!("Trying to get v after ownership moving to closure {:?}", v);

    sub_thread_one.join().expect("Error occur while spawning");
    for i in 1..=5 {
        println!("-> Running --main thread-- {} times", i);
        thread::sleep(Duration::from_millis(1));
    }

    sub_thread_two.join().expect("Error in thread two");
    transfering_vec_closure
        .join()
        .expect("Error on passing ownership");
}

