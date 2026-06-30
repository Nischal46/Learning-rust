use std::{ops::AddAssign, sync::{Arc, Mutex}, thread, time::Duration};

fn main (){
    println!("This is the concept of the rust write in vscode by vim");

    println!("Writing of the thread concept");

    let mutex_data = Arc::new( Mutex::new(0u32));

    let mutex_one = Arc::clone(&mutex_data);
    let thread_one = thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        let mut data = mutex_one.lock().expect("Error in thread one");
        println!("This is thread one");
        *data = 45;
        println!("Logging of mutex guard val in thread one {:?}", data);
    });

    let mutex_two = Arc::clone(&mutex_data);
    let thread_two = thread::spawn(move || {
        let mut data = mutex_two.lock().expect("error in expecting mutex data in thread two");
        data.add_assign(33);
        println!("This is thread two");
        println!("Logging of the mutex val {:?}", data);
        drop(data);
    });

    thread_one.join().unwrap();
    thread_two.join().unwrap();

    println!("At last the mutex holds data as {:?}", mutex_data);
}