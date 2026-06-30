use std::thread;

fn main (){
    println!("This is the concept of the rust write in vscode by vim");

    println!("Writing of the thread concept");

    let thread_one = thread::spawn(|| async {
        println!("This is thread one");
    });

    let thread_two = thread::spawn(|| async {
        println!("This is thread two");
    });

    thread_one.join().unwrap();
    thread_two.join().unwrap();
}