//trying asynchronous coding approach in rust

fn main() {
    let heap_store_string = String::from("Hello, Heap!");

    let clone_string = heap_store_string.clone();

    let handle_for_two_seconds = std::thread::spawn( move || {
        std::thread::sleep(std::time::Duration::from_secs(2));
        accept_string_heap_type(&clone_string);
    });

    println!(
        "Although we pass reference only so we can access it as : {}",
        heap_store_string
    );

    handle_for_two_seconds.join().unwrap();
}

fn accept_string_heap_type(s: &String) {
    println!("{}", s);
}
