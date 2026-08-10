pub fn init() {
    println!("Hello hello from inside of module");
    let mut box_pointer = Box::new(3);
}

fn pass_box_pointer_reference() {
    println!("Here we expect to have get box clone pointer");
}
