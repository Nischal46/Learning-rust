use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Taking time for calculating");
    thread::sleep(Duration::from_secs(3));
    println!("Printing only after 3 seconds");
    intensity
}
fn main(){
    // simulated_expensive_calculation(5);
    // hold_for_four_second();
    normal_function();

    // for n in 0..100000000 {
    //     println!("times: {}", n);
    // }

    let closure_concept = |x| {
        thread::sleep(Duration::from_secs(5));
        println!("Closure concept {}", x);
    };

    closure_concept(10);
    closure_concept(20);
    closure_concept(70);
}

fn hold_for_four_second() {
    thread::sleep(Duration::from_secs(4));
    println!("Hold for-4 Second");
}

fn normal_function(){
    println!("Normal function");
}