pub fn init() {
    println!("This is rust concept for the array manipulation");

    //this is check in compile time
    let mut arr = ["hello", "hi"];
    println!("{}", arr[0]);

    arr[1] = "nischal";
    println!("{:#?}", arr);

    arr[2] = "ban";
    println!(":?", arr);
}
