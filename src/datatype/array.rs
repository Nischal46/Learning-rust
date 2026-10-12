pub fn init() {
    //concept for arrays
    let mut arr1 = [12, 23, 34, 45, 54, 25];

    arr1.sort();
    println!("sorting array: {:?}", arr1);

    arr1.swap(1, 4);
    println!("swapping of the array position: {:?}", arr1);

    match arr1.get(23) {
        Some(x) => println!("element exists"),
        None => println!("element not exists"),
    };
}
