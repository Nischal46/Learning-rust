use std::thread;
use std::time::Duration;

fn alarm_fn() {
    thread::sleep(Duration::from_secs(4));
    println!("Alarm function executes");
}

fn exercise_fn() {
    thread::sleep(Duration::from_secs(2));
    println!("Exercise function executes");
}

pub fn init() {
    println!("This is the concept of thread");

    println!("Before using spawn technique");

    alarm_fn();

    exercise_fn();

    let handle1 = thread::spawn(|| {
        alarm_fn();
    });

    let handle2 = thread::spawn(|| {
        exercise_fn();
    });

    println!("After using of spawn technique that makes to run in parallel");

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("All tasks are completed");
}

